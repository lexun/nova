//! Octree-based 3D LOD system for voxel terrain
//!
//! Provides adaptive level of detail through octree subdivision, enabling:
//! - Unlimited terrain height (vertical chunks)
//! - Caves, tunnels, and overhangs (full 3D)
//! - View-dependent detail (better performance)
//! - Consistent voxel detail in all dimensions

use bevy::prelude::*;
use super::LodLevel;

/// 3D coordinate for octree nodes
///
/// Unlike the 2D RegionCoord, this supports full 3D positioning
/// of chunks at different heights.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OctreeCoord {
    /// X coordinate in octree space
    pub x: i32,
    /// Y coordinate in octree space (vertical)
    pub y: i32,
    /// Z coordinate in octree space
    pub z: i32,
}

impl OctreeCoord {
    /// Create a new octree coordinate
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// Convert world position to octree coordinate at given size
    ///
    /// # Arguments
    /// * `world_pos` - Position in world space (meters)
    /// * `node_size` - Size of each octree node (meters)
    pub fn from_world_pos(world_pos: Vec3, node_size: f32) -> Self {
        Self {
            x: (world_pos.x / node_size).floor() as i32,
            y: (world_pos.y / node_size).floor() as i32,
            z: (world_pos.z / node_size).floor() as i32,
        }
    }

    /// Get the world-space position of this node's origin (corner)
    ///
    /// # Arguments
    /// * `node_size` - Size of each octree node (meters)
    pub fn to_world_pos(&self, node_size: f32) -> Vec3 {
        Vec3::new(
            self.x as f32 * node_size,
            self.y as f32 * node_size,
            self.z as f32 * node_size,
        )
    }

    /// Get the world-space position of this node's center
    ///
    /// # Arguments
    /// * `node_size` - Size of each octree node (meters)
    pub fn to_world_center(&self, node_size: f32) -> Vec3 {
        let half_size = node_size / 2.0;
        self.to_world_pos(node_size) + Vec3::splat(half_size)
    }
}

/// Represents a node in the octree hierarchy
///
/// Each node can either be a leaf (contains a chunk) or a parent
/// with 8 children subdividing the space.
#[derive(Debug, Clone)]
pub struct OctreeNode {
    /// 3D coordinate in octree space
    pub coord: OctreeCoord,

    /// Size of this node in meters
    pub size: f32,

    /// LOD level of this node
    pub lod: LodLevel,

    /// Whether this node is subdivided (has children)
    pub is_subdivided: bool,

    /// Child node coordinates (8 children in 2×2×2 pattern)
    /// Only populated if is_subdivided = true
    pub children: Vec<OctreeCoord>,

    /// Chunk entity if this is a leaf node
    pub chunk_entity: Option<Entity>,
}

impl OctreeNode {
    /// Create a new octree node
    pub fn new(coord: OctreeCoord, size: f32, lod: LodLevel) -> Self {
        Self {
            coord,
            size,
            lod,
            is_subdivided: false,
            children: Vec::new(),
            chunk_entity: None,
        }
    }

    /// Get the world-space bounds of this node
    pub fn bounds(&self) -> (Vec3, Vec3) {
        let min = self.coord.to_world_pos(self.size);
        let max = min + Vec3::splat(self.size);
        (min, max)
    }

    /// Get the center of this node in world space
    pub fn center(&self) -> Vec3 {
        self.coord.to_world_center(self.size)
    }

    /// Calculate distance from a point to the nearest point on this node
    pub fn distance_to(&self, point: Vec3) -> f32 {
        let (min, max) = self.bounds();
        let closest = point.clamp(min, max);
        point.distance(closest)
    }

    /// Subdivide this node into 8 children
    ///
    /// Creates children in a 2×2×2 pattern, each with half the size
    /// and one LOD level higher (more detail).
    pub fn subdivide(&mut self) -> Vec<OctreeCoord> {
        if self.is_subdivided {
            return self.children.clone();
        }

        let _child_size = self.size / 2.0; // For future use in chunk generation
        let _next_lod = match self.lod {
            LodLevel::Lod4 => LodLevel::Lod3,
            LodLevel::Lod3 => LodLevel::Lod2,
            LodLevel::Lod2 => LodLevel::Lod1,
            LodLevel::Lod1 => LodLevel::Lod0,
            LodLevel::Lod0 => LodLevel::Lod0, // Max detail
            LodLevel::None => LodLevel::None,
        }; // For future use in chunk generation

        let mut children = Vec::with_capacity(8);

        // Create 8 children in 2×2×2 pattern
        for dy in 0..2 {
            for dz in 0..2 {
                for dx in 0..2 {
                    let child_coord = OctreeCoord::new(
                        self.coord.x * 2 + dx,
                        self.coord.y * 2 + dy,
                        self.coord.z * 2 + dz,
                    );
                    children.push(child_coord);
                }
            }
        }

        self.children = children.clone();
        self.is_subdivided = true;

        children
    }

    /// Check if this node should be subdivided based on camera distance
    ///
    /// Uses the same distance thresholds as LodLevel
    pub fn should_subdivide(&self, camera_pos: Vec3) -> bool {
        if self.lod == LodLevel::Lod0 {
            return false; // Already at max detail
        }

        let distance = self.distance_to(camera_pos);

        // Should subdivide if we're close enough to need higher detail
        let threshold = match self.lod {
            LodLevel::Lod4 => 300.0,
            LodLevel::Lod3 => 150.0,
            LodLevel::Lod2 => 75.0,
            LodLevel::Lod1 => 35.0,
            LodLevel::Lod0 => f32::MAX,
            LodLevel::None => 400.0,
        };

        distance < threshold
    }

    /// Check if this node should be merged (de-subdivided) based on camera distance
    ///
    /// Uses hysteresis to prevent flickering
    pub fn should_merge(&self, camera_pos: Vec3) -> bool {
        if !self.is_subdivided {
            return false;
        }

        let distance = self.distance_to(camera_pos);
        let hysteresis = 5.0;

        let threshold = match self.lod {
            LodLevel::Lod4 => 300.0 + hysteresis,
            LodLevel::Lod3 => 150.0 + hysteresis,
            LodLevel::Lod2 => 75.0 + hysteresis,
            LodLevel::Lod1 => 35.0 + hysteresis,
            LodLevel::Lod0 => f32::MAX,
            LodLevel::None => 400.0 + hysteresis,
        };

        distance >= threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_octree_coord_creation() {
        let coord = OctreeCoord::new(1, 2, 3);
        assert_eq!(coord.x, 1);
        assert_eq!(coord.y, 2);
        assert_eq!(coord.z, 3);
    }

    #[test]
    fn test_octree_coord_from_world_pos() {
        let world_pos = Vec3::new(16.0, 32.0, 48.0);
        let coord = OctreeCoord::from_world_pos(world_pos, 16.0);
        assert_eq!(coord, OctreeCoord::new(1, 2, 3));
    }

    #[test]
    fn test_octree_coord_to_world_pos() {
        let coord = OctreeCoord::new(2, 3, 4);
        let world_pos = coord.to_world_pos(10.0);
        assert_eq!(world_pos, Vec3::new(20.0, 30.0, 40.0));
    }

    #[test]
    fn test_octree_coord_to_world_center() {
        let coord = OctreeCoord::new(0, 0, 0);
        let center = coord.to_world_center(10.0);
        assert_eq!(center, Vec3::new(5.0, 5.0, 5.0));
    }

    #[test]
    fn test_octree_node_bounds() {
        let node = OctreeNode::new(OctreeCoord::new(1, 2, 3), 10.0, LodLevel::Lod2);
        let (min, max) = node.bounds();
        assert_eq!(min, Vec3::new(10.0, 20.0, 30.0));
        assert_eq!(max, Vec3::new(20.0, 30.0, 40.0));
    }

    #[test]
    fn test_octree_node_center() {
        let node = OctreeNode::new(OctreeCoord::new(0, 0, 0), 16.0, LodLevel::Lod2);
        assert_eq!(node.center(), Vec3::new(8.0, 8.0, 8.0));
    }

    #[test]
    fn test_octree_node_distance_to() {
        let node = OctreeNode::new(OctreeCoord::new(0, 0, 0), 10.0, LodLevel::Lod2);

        // Point inside node
        assert_eq!(node.distance_to(Vec3::new(5.0, 5.0, 5.0)), 0.0);

        // Point outside node
        let distance = node.distance_to(Vec3::new(20.0, 5.0, 5.0));
        assert!((distance - 10.0).abs() < 0.01);
    }

    #[test]
    fn test_octree_node_subdivide() {
        let mut node = OctreeNode::new(OctreeCoord::new(1, 2, 3), 16.0, LodLevel::Lod2);
        assert!(!node.is_subdivided);
        assert_eq!(node.children.len(), 0);

        let children = node.subdivide();
        assert!(node.is_subdivided);
        assert_eq!(children.len(), 8);

        // Check that children are in 2×2×2 pattern
        assert_eq!(children[0], OctreeCoord::new(2, 4, 6)); // 0,0,0
        assert_eq!(children[1], OctreeCoord::new(3, 4, 6)); // 1,0,0
        assert_eq!(children[2], OctreeCoord::new(2, 4, 7)); // 0,0,1
        assert_eq!(children[7], OctreeCoord::new(3, 5, 7)); // 1,1,1
    }

    #[test]
    fn test_octree_node_should_subdivide() {
        let node = OctreeNode::new(OctreeCoord::new(0, 0, 0), 32.0, LodLevel::Lod2);

        // Close camera should trigger subdivision
        assert!(node.should_subdivide(Vec3::new(16.0, 16.0, 16.0)));

        // Far camera should not
        assert!(!node.should_subdivide(Vec3::new(200.0, 200.0, 200.0)));
    }

    #[test]
    fn test_octree_node_max_lod_no_subdivide() {
        let node = OctreeNode::new(OctreeCoord::new(0, 0, 0), 8.0, LodLevel::Lod0);

        // Even close camera shouldn't subdivide max LOD
        assert!(!node.should_subdivide(Vec3::new(4.0, 4.0, 4.0)));
    }

    #[test]
    fn test_octree_node_should_merge() {
        let mut node = OctreeNode::new(OctreeCoord::new(0, 0, 0), 32.0, LodLevel::Lod2);
        node.subdivide();

        // Far camera should trigger merge
        assert!(node.should_merge(Vec3::new(200.0, 200.0, 200.0)));

        // Close camera should not
        assert!(!node.should_merge(Vec3::new(16.0, 16.0, 16.0)));
    }
}
