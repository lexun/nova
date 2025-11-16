//! High-level voxel terrain plugin and components
//!
//! Provides a batteries-included API for spawning voxel terrain with sensible defaults.

use crate::lod::{ChunkManager, LodSettings, RegionCoord};
use crate::terrain::{DefaultTerrainGenerator, TerrainGenerator};
use bevy::prelude::*;

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

    /// Size of each region in meters
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
    /// - 32m regions
    /// - Default gray material
    pub fn planar(world_size: f32) -> Self {
        Self {
            generator: Box::new(DefaultTerrainGenerator::new().with_max_world_size(world_size)),
            lod_settings: LodSettings::demo(),
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
            region_size: 32.0,
            world_bounds: None,
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
            .add_systems(Update, (
                initialize_terrain,
                update_terrain_lod,
                spawn_terrain_chunks,
                cleanup_terrain_chunks,
            ).chain());
    }
}

/// Marker component for terrain that has been initialized
#[derive(Component)]
struct TerrainInitialized;

/// Component tracking chunk entities for a terrain instance
#[derive(Component)]
struct TerrainChunks {
    manager: ChunkManager,
    /// Entities pending despawn
    pending_despawn: Vec<Entity>,
}

/// Initialize newly spawned VoxelTerrain entities
fn initialize_terrain(
    mut commands: Commands,
    query: Query<Entity, (With<VoxelTerrain>, Without<TerrainInitialized>)>,
) {
    for entity in &query {
        commands.entity(entity).insert((
            TerrainInitialized,
            TerrainChunks {
                manager: ChunkManager::new(),
                pending_despawn: Vec::new(),
            },
        ));

        info!("Initialized voxel terrain entity {:?}", entity);
    }
}

/// Update LOD for all terrain based on camera position
fn update_terrain_lod(
    mut terrain_query: Query<(&VoxelTerrain, &mut TerrainChunks)>,
    camera_query: Query<&Transform, With<Camera3d>>,
) {
    // Get camera position (use first camera found)
    let Some(camera_transform) = camera_query.iter().next() else {
        return;
    };
    let camera_pos = camera_transform.translation;

    for (terrain, mut chunks) in &mut terrain_query {
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
                let current_lod = chunks.manager.get_region(region)
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
                        if let Some((_, entities)) = chunks.manager.remove_region(region) {
                            chunks.pending_despawn.extend(entities);
                        }
                    } else if current_lod == LodLevel::None {
                        // Add new region (with empty entity list initially)
                        chunks.manager.add_region(region, target_lod, Vec::new());
                    } else {
                        // Update existing region's LOD - mark old entities for despawn
                        let old_entities: Vec<Entity> = chunks.manager.get_region_mut(region)
                            .map(|(_, entities)| entities.drain(..).collect())
                            .unwrap_or_default();
                        chunks.pending_despawn.extend(old_entities);
                        chunks.manager.update_region_lod(region, target_lod);
                    }
                }
            }
        }
    }
}

/// Spawn chunk meshes for regions that need them
fn spawn_terrain_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut terrain_query: Query<(&VoxelTerrain, &mut TerrainChunks)>,
) {
    for (terrain, mut chunks) in &mut terrain_query {
        let settings = &terrain.lod_settings;
        let region_size = terrain.region_size;

        // Get or create material
        let material = terrain.material.clone().unwrap_or_else(|| {
            materials.add(StandardMaterial {
                base_color: Color::srgb(0.6, 0.6, 0.6),
                perceptual_roughness: 0.8,
                ..default()
            })
        });

        // Find regions that need chunk entities spawned
        let regions_to_spawn: Vec<_> = chunks.manager.regions()
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

                    // Generate mesh
                    let mesh = crate::meshing::generate_chunk_mesh(&chunk, voxel_size);

                    // Spawn entity
                    let entity = commands.spawn((
                        Mesh3d(meshes.add(mesh)),
                        MeshMaterial3d(material.clone()),
                        Transform::from_translation(chunk_offset),
                    )).id();

                    chunk_entities.push(entity);
                }
            }

            // Update chunk manager with spawned entities
            chunks.manager.update_region_chunks(region, chunk_entities);
        }
    }
}

/// Cleanup chunks for regions that were removed
fn cleanup_terrain_chunks(
    mut commands: Commands,
    mut terrain_query: Query<&mut TerrainChunks>,
) {
    for mut chunks in &mut terrain_query {
        // Despawn all pending entities
        for entity in chunks.pending_despawn.drain(..) {
            commands.entity(entity).despawn();
        }
    }
}
