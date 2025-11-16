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

/// Manages the octree hierarchy for 3D LOD terrain
///
/// Tracks active octree nodes and their subdivision state based on camera position.
/// Nodes automatically subdivide when the camera gets close and merge when it moves away.
#[derive(Resource)]
pub struct OctreeManager {
    /// All active octree nodes indexed by coordinate
    nodes: std::collections::HashMap<OctreeCoord, OctreeNode>,

    /// Root node size (largest octree node size)
    root_size: f32,

    /// Entities pending visibility change (to be hidden, not despawned)
    pending_hide: Vec<Entity>,

    /// Entities pending despawn (only when truly far away)
    pending_despawn: Vec<Entity>,

    /// Cache of entity handles by coordinate to avoid regeneration
    /// When nodes are hidden, they stay in this cache for fast reactivation
    entity_cache: std::collections::HashMap<OctreeCoord, Entity>,
}

impl Default for OctreeManager {
    fn default() -> Self {
        Self {
            nodes: std::collections::HashMap::new(),
            root_size: 64.0,
            pending_hide: Vec::new(),
            pending_despawn: Vec::new(),
            entity_cache: std::collections::HashMap::new(),
        }
    }
}

impl OctreeManager {
    /// Create a new octree manager with given root node size
    ///
    /// # Arguments
    /// * `root_size` - Size of the largest octree nodes (e.g., 64m)
    pub fn new(root_size: f32) -> Self {
        Self {
            nodes: std::collections::HashMap::new(),
            root_size,
            pending_hide: Vec::new(),
            pending_despawn: Vec::new(),
            entity_cache: std::collections::HashMap::new(),
        }
    }

    /// Get a node by coordinate
    pub fn get_node(&self, coord: OctreeCoord) -> Option<&OctreeNode> {
        self.nodes.get(&coord)
    }

    /// Get a mutable node by coordinate
    pub fn get_node_mut(&mut self, coord: OctreeCoord) -> Option<&mut OctreeNode> {
        self.nodes.get_mut(&coord)
    }

    /// Insert or update a node
    pub fn insert_node(&mut self, node: OctreeNode) {
        self.nodes.insert(node.coord, node);
    }

    /// Remove a node and return it
    pub fn remove_node(&mut self, coord: OctreeCoord) -> Option<OctreeNode> {
        self.nodes.remove(&coord)
    }

    /// Check if a node exists
    pub fn has_node(&self, coord: OctreeCoord) -> bool {
        self.nodes.contains_key(&coord)
    }

    /// Get the number of active nodes
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get iterator over all nodes
    pub fn nodes(&self) -> impl Iterator<Item = (&OctreeCoord, &OctreeNode)> {
        self.nodes.iter()
    }

    /// Update octree based on camera position
    ///
    /// Subdivides nodes that are close to the camera and merges nodes that are far away.
    /// Returns a list of coordinates that need chunks spawned.
    pub fn update(&mut self, camera_pos: Vec3) -> Vec<OctreeCoord> {
        let mut coords_to_spawn = Vec::new();

        // Find nodes that need subdivision
        let to_subdivide: Vec<OctreeCoord> = self.nodes.values()
            .filter(|node| !node.is_subdivided && node.should_subdivide(camera_pos))
            .map(|node| node.coord)
            .collect();

        // Subdivide nodes
        for coord in to_subdivide {
            let (entity_to_hide, children, child_size, child_lod) = if let Some(node) = self.nodes.get_mut(&coord) {
                // Mark old chunk for hiding (not despawning - keep it cached)
                let entity_to_hide = node.chunk_entity.take();

                // Create children
                let children = node.subdivide();
                let child_size = node.size / 2.0;
                let child_lod = match node.lod {
                    LodLevel::Lod4 => LodLevel::Lod3,
                    LodLevel::Lod3 => LodLevel::Lod2,
                    LodLevel::Lod2 => LodLevel::Lod1,
                    LodLevel::Lod1 => LodLevel::Lod0,
                    LodLevel::Lod0 => LodLevel::Lod0,
                    LodLevel::None => LodLevel::None,
                };

                (entity_to_hide, children, child_size, child_lod)
            } else {
                continue;
            };

            // Handle entity hiding (outside of mutable borrow)
            if let Some(entity) = entity_to_hide {
                self.pending_hide.push(entity);
                self.cache_entity(coord, entity);
            }

            // Add child nodes
            for child_coord in children {
                let child_node = OctreeNode::new(child_coord, child_size, child_lod);
                coords_to_spawn.push(child_coord);
                self.nodes.insert(child_coord, child_node);
            }
        }

        // Find nodes that should merge
        let to_merge: Vec<OctreeCoord> = self.nodes.values()
            .filter(|node| node.is_subdivided && node.should_merge(camera_pos))
            .map(|node| node.coord)
            .collect();

        // Merge nodes (remove children)
        for coord in to_merge {
            if let Some(node) = self.nodes.get(&coord) {
                let children = node.children.clone();

                // Remove all children and mark their chunks for hiding (keep cached)
                for child_coord in children {
                    if let Some(child) = self.remove_node(child_coord) {
                        if let Some(entity) = child.chunk_entity {
                            self.pending_hide.push(entity);
                            // Cache child entities
                            self.cache_entity(child_coord, entity);
                        }
                    }
                }

                // Mark parent as not subdivided
                if let Some(parent) = self.nodes.get_mut(&coord) {
                    parent.is_subdivided = false;
                    parent.children.clear();
                    // Parent needs a chunk spawned now
                    coords_to_spawn.push(coord);
                }
            }
        }

        coords_to_spawn
    }

    /// Take all entities pending despawn
    pub fn take_pending_despawn(&mut self) -> Vec<Entity> {
        std::mem::take(&mut self.pending_despawn)
    }

    /// Take all entities pending hide (should be made invisible)
    pub fn take_pending_hide(&mut self) -> Vec<Entity> {
        std::mem::take(&mut self.pending_hide)
    }

    /// Get cached entity for a coordinate (if it exists)
    pub fn get_cached_entity(&self, coord: OctreeCoord) -> Option<Entity> {
        self.entity_cache.get(&coord).copied()
    }

    /// Cache an entity for a coordinate
    pub fn cache_entity(&mut self, coord: OctreeCoord, entity: Entity) {
        self.entity_cache.insert(coord, entity);
    }

    /// Remove entity from cache
    pub fn uncache_entity(&mut self, coord: OctreeCoord) -> Option<Entity> {
        self.entity_cache.remove(&coord)
    }

    /// Initialize octree with root nodes around a center point
    ///
    /// Creates a grid of root nodes to cover a region around the camera.
    pub fn initialize_around_point(&mut self, center: Vec3, radius: f32) {
        let grid_radius = (radius / self.root_size).ceil() as i32;
        let center_coord = OctreeCoord::from_world_pos(center, self.root_size);

        for dy in -grid_radius..=grid_radius {
            for dz in -grid_radius..=grid_radius {
                for dx in -grid_radius..=grid_radius {
                    let coord = OctreeCoord::new(
                        center_coord.x + dx,
                        center_coord.y + dy,
                        center_coord.z + dz,
                    );

                    if !self.has_node(coord) {
                        let node = OctreeNode::new(coord, self.root_size, LodLevel::Lod4);
                        self.insert_node(node);
                    }
                }
            }
        }
    }

    /// Clear all nodes
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.pending_despawn.clear();
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
