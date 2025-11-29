//! High-level voxel terrain plugin and components
//!
//! Provides a batteries-included API for spawning voxel terrain with sensible defaults.

use crate::lod::{ChunkManager, LodSettings, OctreeManager, RegionCoord};
use crate::terrain::{DefaultTerrainGenerator, TerrainGenerator};
use bevy::prelude::*;

/// LOD strategy selection for terrain rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LodStrategy {
    /// 2D heightmap-based LOD (current, proven)
    /// - Fixed concentric rings around camera
    /// - Chunks anchored at Y=0
    /// - No caves or overhangs
    /// - Limited height (32 voxels max)
    Heightmap,

    /// 3D octree-based LOD (new, experimental)
    /// - View-dependent adaptive subdivision
    /// - Chunks at any Y position
    /// - Supports caves, tunnels, overhangs
    /// - Unlimited height
    Octree,
}

/// High-level voxel terrain component
///
/// Spawn this to create voxel terrain with automatic LOD management.
///
/// # Example
/// ```ignore
/// commands.spawn(VoxelTerrain::planar(512.0));
/// ```
#[derive(Component)]
pub struct VoxelTerrain {
    /// Terrain generator
    pub generator: Box<dyn TerrainGenerator>,

    /// LOD settings (distances, voxel sizes, hysteresis)
    pub lod_settings: LodSettings,

    /// LOD strategy (heightmap or octree)
    pub lod_strategy: LodStrategy,

    /// Size of each region/octree node in meters
    pub region_size: f32,

    /// World bounds (for finite terrain)
    /// None = infinite terrain
    pub world_bounds: Option<Vec2>,

    /// Material handle for rendering chunks
    pub material: Option<Handle<StandardMaterial>>,
}

impl VoxelTerrain {
    /// Create planar terrain with given world size
    ///
    /// Uses sensible defaults:
    /// - DefaultTerrainGenerator (rolling hills)
    /// - LodSettings::demo() (400m view distance)
    /// - Heightmap strategy (2D, proven)
    /// - 32m regions
    /// - Default gray material
    pub fn planar(world_size: f32) -> Self {
        Self {
            generator: Box::new(DefaultTerrainGenerator::new().with_max_world_size(world_size)),
            lod_settings: LodSettings::demo(),
            lod_strategy: LodStrategy::Heightmap,
            region_size: 32.0,
            world_bounds: Some(Vec2::splat(world_size)),
            material: None,
        }
    }

    /// Create infinite planar terrain (no bounds)
    pub fn infinite() -> Self {
        Self {
            generator: Box::new(DefaultTerrainGenerator::new()),
            lod_settings: LodSettings::infinite(),
            lod_strategy: LodStrategy::Heightmap,
            region_size: 32.0,
            world_bounds: None,
            material: None,
        }
    }

    /// Create 3D cubic terrain using octree LOD
    ///
    /// Enables caves, tunnels, and unlimited height.
    /// Uses octree-based adaptive LOD for better performance.
    pub fn cubic(world_size: f32) -> Self {
        Self {
            generator: Box::new(DefaultTerrainGenerator::new().with_max_world_size(world_size)),
            lod_settings: LodSettings::demo(),
            lod_strategy: LodStrategy::Octree,
            region_size: 64.0, // Larger root nodes for octree
            world_bounds: Some(Vec2::splat(world_size)),
            material: None,
        }
    }

    /// Set custom terrain generator
    pub fn with_generator(mut self, generator: impl TerrainGenerator + 'static) -> Self {
        self.generator = Box::new(generator);
        self
    }

    /// Set LOD settings
    pub fn with_lod_settings(mut self, settings: LodSettings) -> Self {
        self.lod_settings = settings;
        self
    }

    /// Set region size
    pub fn with_region_size(mut self, size: f32) -> Self {
        self.region_size = size;
        self
    }

    /// Set material
    pub fn with_material(mut self, material: Handle<StandardMaterial>) -> Self {
        self.material = Some(material);
        self
    }
}

/// Plugin for automatic voxel terrain management
///
/// Registers systems for:
/// - Initializing terrain on spawn
/// - Updating LOD based on camera distance
/// - Spawning/despawning chunk meshes
pub struct VoxelTerrainPlugin;

impl Plugin for VoxelTerrainPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_texture_atlas)
            .add_systems(Update, (
                initialize_terrain,
                update_terrain_lod,
                spawn_terrain_chunks,
                cleanup_terrain_chunks,
            ).chain());
    }
}

/// Resource holding the material handles
#[derive(Resource)]
struct VoxelMaterials {
    materials: std::collections::HashMap<crate::meshing::MaterialKey, Handle<StandardMaterial>>,
}

/// Setup material textures on startup
fn setup_texture_atlas(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    use crate::atlas::FaceDir;
    use crate::meshing::MaterialKey;
    use crate::VoxelType;

    info!("Generating voxel material textures...");
    let textures = crate::textures::generate_material_textures(&mut images);

    // Create StandardMaterial for each possible voxel_type + face_dir combination
    let mut material_map = std::collections::HashMap::new();

    for voxel_type in [VoxelType::Stone, VoxelType::Dirt, VoxelType::Grass] {
        for face_dir in [FaceDir::Top, FaceDir::Bottom, FaceDir::Side] {
            let texture = textures.get_texture(voxel_type, face_dir);
            let material = materials.add(StandardMaterial {
                base_color_texture: Some(texture),
                perceptual_roughness: 0.8,
                ..default()
            });
            material_map.insert(MaterialKey::new(voxel_type, face_dir), material);
        }
    }

    commands.insert_resource(VoxelMaterials {
        materials: material_map,
    });
}

/// Marker component for terrain that has been initialized
#[derive(Component)]
struct TerrainInitialized;

/// LOD manager for a terrain instance
#[derive(Component)]
enum TerrainLodManager {
    Heightmap {
        manager: ChunkManager,
        pending_despawn: Vec<Entity>,
    },
    Octree {
        manager: OctreeManager,
    },
}

/// Initialize newly spawned VoxelTerrain entities
fn initialize_terrain(
    mut commands: Commands,
    query: Query<(Entity, &VoxelTerrain), Without<TerrainInitialized>>,
) {
    for (entity, terrain) in &query {
        let lod_manager = match terrain.lod_strategy {
            LodStrategy::Heightmap => TerrainLodManager::Heightmap {
                manager: ChunkManager::new(),
                pending_despawn: Vec::new(),
            },
            LodStrategy::Octree => TerrainLodManager::Octree {
                manager: OctreeManager::new(terrain.region_size),
            },
        };

        commands.entity(entity).insert((
            TerrainInitialized,
            lod_manager,
        ));

        info!("Initialized voxel terrain entity {:?} with {:?} strategy",
              entity, terrain.lod_strategy);
    }
}

/// Update LOD for all terrain based on camera position
fn update_terrain_lod(
    mut terrain_query: Query<(&VoxelTerrain, &mut TerrainLodManager)>,
    camera_query: Query<&Transform, With<Camera3d>>,
) {
    // Get camera position (use first camera found)
    let Some(camera_transform) = camera_query.iter().next() else {
        return;
    };
    let camera_pos = camera_transform.translation;

    for (terrain, mut lod_manager) in &mut terrain_query {
        match &mut *lod_manager {
            TerrainLodManager::Heightmap { manager: chunks, pending_despawn } => {
                update_heightmap_lod(terrain, chunks, pending_despawn, camera_pos);
            },
            TerrainLodManager::Octree { manager } => {
                // Octree update happens in update() method
                // For now, just initialize if empty
                if manager.node_count() == 0 {
                    manager.initialize_around_point(camera_pos, terrain.lod_settings.max_view_distance);
                }
                // Update will happen in spawn system
            },
        }
    }
}

/// Update heightmap-based LOD
fn update_heightmap_lod(
    terrain: &VoxelTerrain,
    chunks: &mut ChunkManager,
    pending_despawn: &mut Vec<Entity>,
    camera_pos: Vec3,
) {
        let settings = &terrain.lod_settings;
        let region_size = terrain.region_size;

        // Calculate check radius based on max view distance
        let max_distance = settings.max_view_distance;
        let check_radius = ((max_distance + settings.hysteresis) / region_size).ceil() as i32 + 1;

        // Calculate camera's region coordinate
        let camera_region = RegionCoord::from_world_pos(camera_pos, region_size);

        // Check all regions within view distance
        for offset_x in -check_radius..=check_radius {
            for offset_z in -check_radius..=check_radius {
                let region = RegionCoord {
                    x: camera_region.x + offset_x,
                    z: camera_region.z + offset_z,
                };

                // Skip if outside world bounds
                if let Some(bounds) = terrain.world_bounds {
                    let world_x = region.x as f32 * region_size;
                    let world_z = region.z as f32 * region_size;
                    if world_x < 0.0 || world_z < 0.0 ||
                       world_x >= bounds.x || world_z >= bounds.y {
                        continue;
                    }
                }

                // Calculate distance from camera to region center
                let region_center = region.to_world_pos(region_size) + Vec3::new(
                    region_size / 2.0,
                    0.0,
                    region_size / 2.0,
                );
                let distance = camera_pos.distance(region_center);

                // Determine target LOD level
                use crate::lod::LodLevel;
                let current_lod = chunks.get_region(region)
                    .map(|(lod, _)| *lod)
                    .unwrap_or(LodLevel::None);

                let target_lod = LodLevel::from_distance_with_hysteresis_settings(
                    distance,
                    current_lod,
                    settings,
                );

                // Update if LOD changed
                if target_lod != current_lod {
                    if target_lod == LodLevel::None {
                        // Remove region and mark entities for despawn
                        if let Some((_, entities)) = chunks.remove_region(region) {
                            pending_despawn.extend(entities);
                        }
                    } else if current_lod == LodLevel::None {
                        // Add new region (with empty entity list initially)
                        chunks.add_region(region, target_lod, Vec::new());
                    } else {
                        // Update existing region's LOD - mark old entities for despawn
                        let old_entities: Vec<Entity> = chunks.get_region_mut(region)
                            .map(|(_, entities)| entities.drain(..).collect())
                            .unwrap_or_default();
                        pending_despawn.extend(old_entities);
                        chunks.update_region_lod(region, target_lod);
                    }
                }
            }
        }
}

/// Spawn chunk meshes for regions that need them
fn spawn_terrain_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    voxel_materials: Option<Res<VoxelMaterials>>,
    mut terrain_query: Query<(&VoxelTerrain, &mut TerrainLodManager)>,
    camera_query: Query<&Transform, With<Camera3d>>,
) {
    let Some(camera_transform) = camera_query.iter().next() else {
        return;
    };
    let camera_pos = camera_transform.translation;

    for (terrain, mut lod_manager) in &mut terrain_query {
        match &mut *lod_manager {
            TerrainLodManager::Heightmap { manager: chunks, .. } => {
                spawn_heightmap_chunks(terrain, chunks, &voxel_materials, &mut commands, &mut meshes);
            },
            TerrainLodManager::Octree { manager } => {
                spawn_octree_chunks(terrain, manager, camera_pos, &voxel_materials, &mut commands, &mut meshes);
            },
        }
    }
}

/// Spawn heightmap chunks
fn spawn_heightmap_chunks(
    terrain: &VoxelTerrain,
    chunks: &mut ChunkManager,
    voxel_materials: &Option<Res<VoxelMaterials>>,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    let settings = &terrain.lod_settings;
    let region_size = terrain.region_size;

    // Get material map or warn if not available
    let Some(voxel_materials_res) = voxel_materials else {
        warn!("No material available for terrain chunks - materials not initialized yet");
        return;
    };

    // Find regions that need chunk entities spawned
    let regions_to_spawn: Vec<_> = chunks.regions()
        .filter(|(_, data)| data.1.is_empty())
        .map(|(region, data)| (*region, data.0))
        .collect();

    for (region, lod) in regions_to_spawn {
        let voxel_size = lod.voxel_size_with_settings(settings);
        let chunks_per_edge = lod.chunks_per_edge();

        let region_world_pos = region.to_world_pos(region_size);
        let chunk_world_size = region_size / chunks_per_edge as f32;

        let mut chunk_entities = Vec::new();

        // Generate chunks for this region
        for chunk_x in 0..chunks_per_edge {
            for chunk_z in 0..chunks_per_edge {
                let chunk_offset = region_world_pos + Vec3::new(
                    chunk_x as f32 * chunk_world_size,
                    0.0,
                    chunk_z as f32 * chunk_world_size,
                );

                // Generate voxel data
                let chunk = terrain.generator.generate_chunk(chunk_offset, voxel_size);

                // Generate multiple meshes grouped by material
                let meshes_by_material = crate::meshing::generate_chunk_meshes(&chunk, voxel_size);

                // Spawn one entity per material
                for (material_key, mesh) in meshes_by_material {
                    if let Some(material) = voxel_materials_res.materials.get(&material_key) {
                        let entity = commands.spawn((
                            Mesh3d(meshes.add(mesh)),
                            MeshMaterial3d(material.clone()),
                            Transform::from_translation(chunk_offset),
                        )).id();

                        chunk_entities.push(entity);
                    }
                }
            }
        }

        // Update chunk manager with spawned entities
        chunks.update_region_chunks(region, chunk_entities);
    }
}

/// Spawn octree chunks
fn spawn_octree_chunks(
    terrain: &VoxelTerrain,
    octree: &mut OctreeManager,
    camera_pos: Vec3,
    voxel_materials: &Option<Res<VoxelMaterials>>,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    // Update octree (subdivide/merge based on camera)
    let _coords_to_spawn = octree.update(camera_pos);

    // Get material map or warn if not available
    let Some(voxel_materials_res) = voxel_materials else {
        warn!("No material available for terrain chunks - materials not initialized yet");
        return;
    };

    // Spawn chunks for leaf nodes that don't have entities
    let nodes_to_spawn: Vec<_> = octree.nodes()
        .filter(|(_, node)| !node.is_subdivided && node.chunk_entity.is_none())
        .map(|(coord, node)| (*coord, node.size, node.lod))
        .collect();

    for (coord, size, lod) in nodes_to_spawn {
        let voxel_size = lod.voxel_size_with_settings(&terrain.lod_settings);
        let world_pos = coord.to_world_pos(size);

        // Generate voxel data
        let chunk = terrain.generator.generate_chunk(world_pos, voxel_size);

        // Generate multiple meshes grouped by material
        let meshes_by_material = crate::meshing::generate_chunk_meshes(&chunk, voxel_size);

        // Spawn one entity per material (only store first entity in octree node)
        let mut first_entity = None;
        for (material_key, mesh) in meshes_by_material {
            if let Some(material) = voxel_materials_res.materials.get(&material_key) {
                let entity = commands.spawn((
                    Mesh3d(meshes.add(mesh)),
                    MeshMaterial3d(material.clone()),
                    Transform::from_translation(world_pos),
                )).id();

                if first_entity.is_none() {
                    first_entity = Some(entity);
                }
            }
        }

        // Store first entity in octree node (octree only tracks one entity per node)
        // TODO: Update octree to track multiple entities per node
        if let Some(node) = octree.get_node_mut(coord) {
            node.chunk_entity = first_entity;
        }
    }
}

/// Cleanup chunks for regions that were removed
fn cleanup_terrain_chunks(
    mut commands: Commands,
    mut terrain_query: Query<&mut TerrainLodManager>,
) {
    for mut lod_manager in &mut terrain_query {
        match &mut *lod_manager {
            TerrainLodManager::Heightmap { pending_despawn, .. } => {
                for entity in pending_despawn.drain(..) {
                    commands.entity(entity).despawn();
                }
            },
            TerrainLodManager::Octree { manager } => {
                for entity in manager.take_pending_despawn() {
                    commands.entity(entity).despawn();
                }
            },
        }
    }
}
