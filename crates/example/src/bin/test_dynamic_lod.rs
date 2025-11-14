//! # Test: Dynamic LOD Terrain System
//!
//! Demonstrates real-time Level of Detail (LOD) transitions based on camera distance.
//! Large terrain area where chunks spawn/despawn dynamically as you move around.
//!
//! ## LOD Strategy
//!
//! - **High LOD** (0-50m): 0.25m voxels, 16 chunks per region (4×4 grid)
//! - **Medium LOD** (50-150m): 1.0m voxels, 4 chunks per region (2×2 grid)
//! - **Low LOD** (>150m): 4.0m voxels, 1 chunk per region
//!
//! ## Controls
//!
//! - WASD: Move camera
//! - QE: Up/Down
//! - Mouse: Look around
//! - F3: Toggle debug info (shows active chunk count, LOD levels)

use bevy::{
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
};
use bevy_brp_extras::BrpExtrasPlugin;
use engine::{FlyCameraController, FlyCameraPlugin};
use std::collections::HashMap;
use voxel::{Chunk, Voxel, VoxelType, CHUNK_SIZE};

// World configuration
const WORLD_SIZE: f32 = 256.0; // 256m × 256m world
const REGION_SIZE: f32 = 32.0; // Each region is 32m × 32m

// LOD configuration
const LOD_HIGH_DISTANCE: f32 = 50.0;
const LOD_MEDIUM_DISTANCE: f32 = 150.0;
const LOD_HIGH_VOXEL_SIZE: f32 = 0.25;
const LOD_MEDIUM_VOXEL_SIZE: f32 = 1.0;
const LOD_LOW_VOXEL_SIZE: f32 = 4.0;

// Hysteresis buffer to prevent flickering (10% buffer)
const HYSTERESIS_BUFFER: f32 = 5.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RegionCoord {
    x: i32,
    z: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LodLevel {
    High,   // 0.25m voxels, 4×4 chunks
    Medium, // 1.0m voxels, 2×2 chunks
    Low,    // 4.0m voxels, 1 chunk
    None,   // Too far, not rendered
}

impl LodLevel {
    fn from_distance(distance: f32) -> Self {
        if distance < LOD_HIGH_DISTANCE {
            LodLevel::High
        } else if distance < LOD_MEDIUM_DISTANCE {
            LodLevel::Medium
        } else if distance < LOD_MEDIUM_DISTANCE + 50.0 {
            // Render a bit beyond medium distance
            LodLevel::Low
        } else {
            LodLevel::None
        }
    }

    /// Get the target LOD with hysteresis to prevent flickering
    fn from_distance_with_hysteresis(distance: f32, current: LodLevel) -> Self {
        match current {
            LodLevel::None => {
                // Coming into view - use regular thresholds
                Self::from_distance(distance)
            }
            LodLevel::Low => {
                if distance < LOD_MEDIUM_DISTANCE - HYSTERESIS_BUFFER {
                    LodLevel::Medium
                } else if distance > LOD_MEDIUM_DISTANCE + 50.0 + HYSTERESIS_BUFFER {
                    LodLevel::None
                } else {
                    LodLevel::Low
                }
            }
            LodLevel::Medium => {
                if distance < LOD_HIGH_DISTANCE - HYSTERESIS_BUFFER {
                    LodLevel::High
                } else if distance > LOD_MEDIUM_DISTANCE + HYSTERESIS_BUFFER {
                    LodLevel::Low
                } else {
                    LodLevel::Medium
                }
            }
            LodLevel::High => {
                if distance > LOD_HIGH_DISTANCE + HYSTERESIS_BUFFER {
                    LodLevel::Medium
                } else {
                    LodLevel::High
                }
            }
        }
    }

    fn voxel_size(&self) -> f32 {
        match self {
            LodLevel::High => LOD_HIGH_VOXEL_SIZE,
            LodLevel::Medium => LOD_MEDIUM_VOXEL_SIZE,
            LodLevel::Low => LOD_LOW_VOXEL_SIZE,
            LodLevel::None => 0.0,
        }
    }

    fn chunks_per_edge(&self) -> usize {
        match self {
            LodLevel::High => 4,   // 4×4 = 16 chunks
            LodLevel::Medium => 2, // 2×2 = 4 chunks
            LodLevel::Low => 1,    // 1 chunk
            LodLevel::None => 0,
        }
    }

    fn color(&self) -> Color {
        match self {
            LodLevel::High => Color::srgb(0.3, 0.7, 0.2),   // Green
            LodLevel::Medium => Color::srgb(0.2, 0.6, 0.8), // Blue
            LodLevel::Low => Color::srgb(0.7, 0.4, 0.2),    // Brown
            LodLevel::None => Color::BLACK,
        }
    }
}

#[derive(Component)]
struct RegionChunks {
    region: RegionCoord,
    lod: LodLevel,
}

#[derive(Resource, Default)]
struct ChunkManager {
    // Track which regions are loaded and at what LOD
    active_regions: HashMap<RegionCoord, (LodLevel, Vec<Entity>)>,
}

fn setup_scene(mut commands: Commands) {
    // Camera positioned to view terrain
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(WORLD_SIZE / 2.0, 50.0, WORLD_SIZE / 2.0)
            .looking_at(Vec3::new(WORLD_SIZE / 2.0, 0.0, WORLD_SIZE / 2.0 + 50.0), Vec3::Y),
        FlyCameraController::new(25.0, 0.003),
    ));

    // Directional light (sun)
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.7, 0.4, 0.0)),
    ));

    info!("Dynamic LOD terrain system initialized");
    info!("World size: {}m × {}m", WORLD_SIZE, WORLD_SIZE);
    info!("Region size: {}m × {}m", REGION_SIZE, REGION_SIZE);
}

/// System that updates LOD based on camera position
fn update_lod(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chunk_manager: ResMut<ChunkManager>,
    camera_query: Query<&Transform, With<Camera3d>>,
    _chunk_query: Query<Entity, With<RegionChunks>>,
) {
    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    let camera_pos = camera_transform.translation;

    // Calculate which regions are visible
    let camera_region_x = (camera_pos.x / REGION_SIZE).floor() as i32;
    let camera_region_z = (camera_pos.z / REGION_SIZE).floor() as i32;

    // Check regions in a radius around camera
    let check_radius = ((LOD_MEDIUM_DISTANCE + 50.0 + HYSTERESIS_BUFFER) / REGION_SIZE).ceil() as i32 + 1;

    let mut desired_regions: HashMap<RegionCoord, LodLevel> = HashMap::new();

    for region_x in (camera_region_x - check_radius)..=(camera_region_x + check_radius) {
        for region_z in (camera_region_z - check_radius)..=(camera_region_z + check_radius) {
            // Skip regions outside world bounds
            let world_regions = (WORLD_SIZE / REGION_SIZE) as i32;
            if region_x < 0 || region_x >= world_regions || region_z < 0 || region_z >= world_regions {
                continue;
            }

            let region = RegionCoord {
                x: region_x,
                z: region_z,
            };

            // Calculate distance from camera to region center
            let region_center = Vec3::new(
                region_x as f32 * REGION_SIZE + REGION_SIZE / 2.0,
                0.0,
                region_z as f32 * REGION_SIZE + REGION_SIZE / 2.0,
            );

            let distance = camera_pos.distance(region_center);

            // Get current LOD for this region (if any)
            let current_lod = chunk_manager
                .active_regions
                .get(&region)
                .map(|(lod, _)| *lod)
                .unwrap_or(LodLevel::None);

            // Calculate desired LOD with hysteresis
            let desired_lod = LodLevel::from_distance_with_hysteresis(distance, current_lod);

            if desired_lod != LodLevel::None {
                desired_regions.insert(region, desired_lod);
            }
        }
    }

    // Despawn regions that are no longer needed or need LOD change
    let mut regions_to_remove = Vec::new();
    for (region, (current_lod, entities)) in chunk_manager.active_regions.iter() {
        let needs_update = match desired_regions.get(region) {
            Some(desired_lod) => *desired_lod != *current_lod,
            None => true, // Region no longer visible
        };

        if needs_update {
            // Despawn all chunk entities for this region
            for entity in entities {
                commands.entity(*entity).despawn();
            }
            regions_to_remove.push(*region);
        }
    }

    for region in regions_to_remove {
        chunk_manager.active_regions.remove(&region);
    }

    // Spawn new regions or update existing ones
    for (region, desired_lod) in desired_regions {
        if !chunk_manager.active_regions.contains_key(&region) {
            // Spawn new region at desired LOD
            let entities = spawn_region(
                &mut commands,
                &mut meshes,
                &mut materials,
                region,
                desired_lod,
            );
            chunk_manager
                .active_regions
                .insert(region, (desired_lod, entities));
        }
    }
}

/// Spawn all chunks for a region at the specified LOD level
fn spawn_region(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    region: RegionCoord,
    lod: LodLevel,
) -> Vec<Entity> {
    let voxel_size = lod.voxel_size();
    let chunks_per_edge = lod.chunks_per_edge();
    let chunk_world_size = CHUNK_SIZE as f32 * voxel_size;

    let region_world_x = region.x as f32 * REGION_SIZE;
    let region_world_z = region.z as f32 * REGION_SIZE;

    let material = materials.add(StandardMaterial {
        base_color: lod.color(),
        perceptual_roughness: 0.9,
        ..default()
    });

    let mut entities = Vec::new();

    for chunk_x in 0..chunks_per_edge {
        for chunk_z in 0..chunks_per_edge {
            let world_offset_x = region_world_x + (chunk_x as f32 * chunk_world_size);
            let world_offset_z = region_world_z + (chunk_z as f32 * chunk_world_size);

            let chunk = generate_terrain_chunk(world_offset_x, world_offset_z, voxel_size, REGION_SIZE);
            let mesh = voxel::meshing::generate_chunk_mesh(&chunk, voxel_size);

            let position = Vec3::new(world_offset_x, 0.0, world_offset_z);

            let entity = commands
                .spawn((
                    Mesh3d(meshes.add(mesh)),
                    MeshMaterial3d(material.clone()),
                    Transform::from_translation(position),
                    Wireframe,
                    RegionChunks { region, lod },
                ))
                .id();

            entities.push(entity);
        }
    }

    entities
}

/// Generate a terrain chunk at a given world offset
fn generate_terrain_chunk(
    world_offset_x: f32,
    world_offset_z: f32,
    voxel_size: f32,
    max_world_size: f32,
) -> Chunk {
    let mut chunk = Chunk::new();

    for local_x in 0..CHUNK_SIZE {
        for local_z in 0..CHUNK_SIZE {
            let world_x = world_offset_x + (local_x as f32 * voxel_size);
            let world_z = world_offset_z + (local_z as f32 * voxel_size);

            // Skip voxels outside the target world area
            if world_x >= world_offset_x + max_world_size || world_z >= world_offset_z + max_world_size {
                continue;
            }

            // Get terrain height in world space (meters)
            let height_meters = terrain_height(world_x, world_z);

            // Convert to voxel coordinates
            let height_voxels = (height_meters / voxel_size) as usize;
            let max_height = height_voxels.min(CHUNK_SIZE - 1);

            // Fill voxels from bottom to height
            for local_y in 0..=max_height {
                chunk.set_voxel(
                    local_x,
                    local_y,
                    local_z,
                    Voxel {
                        voxel_type: VoxelType::Stone,
                        density: 255,
                    },
                );
            }
        }
    }

    chunk
}

/// Calculate terrain height at world coordinates (scale-independent)
fn terrain_height(world_x: f32, world_z: f32) -> f32 {
    // Base height
    let base = 2.0;

    // Large rolling hills (spans entire world)
    let hills = (world_x * 0.02).sin() * (world_z * 0.02).cos() * 8.0;

    // Medium frequency variation
    let medium = (world_x * 0.1 + world_z * 0.08).sin() * 3.0;

    // Small detail
    let detail = (world_x * 0.4).cos() * (world_z * 0.35).sin() * 1.0;

    // Combine and ensure positive
    (base + hills + medium + detail).max(0.5)
}

/// Debug UI system
fn debug_ui(
    chunk_manager: Res<ChunkManager>,
    camera_query: Query<&Transform, With<Camera3d>>,
) {
    if let Ok(camera_transform) = camera_query.single() {
        let camera_pos = camera_transform.translation;

        let total_chunks: usize = chunk_manager
            .active_regions
            .values()
            .map(|(_, entities)| entities.len())
            .sum();

        let high_count = chunk_manager
            .active_regions
            .values()
            .filter(|(lod, _)| *lod == LodLevel::High)
            .count();

        let medium_count = chunk_manager
            .active_regions
            .values()
            .filter(|(lod, _)| *lod == LodLevel::Medium)
            .count();

        let low_count = chunk_manager
            .active_regions
            .values()
            .filter(|(lod, _)| *lod == LodLevel::Low)
            .count();

        info!(
            "Pos: ({:.1}, {:.1}, {:.1}) | Regions: {} (H:{} M:{} L:{}) | Chunks: {}",
            camera_pos.x, camera_pos.y, camera_pos.z,
            chunk_manager.active_regions.len(),
            high_count, medium_count, low_count,
            total_chunks
        );
    }
}

fn main() {
    let mut app = engine::create_app();
    app.add_plugins((BrpExtrasPlugin, WireframePlugin::default(), FlyCameraPlugin));
    app.init_resource::<ChunkManager>();
    app.add_systems(Startup, setup_scene);
    app.add_systems(Update, update_lod);
    app.add_systems(Update, debug_ui.run_if(bevy::time::common_conditions::on_timer(std::time::Duration::from_secs(2))));
    app.run();
}
