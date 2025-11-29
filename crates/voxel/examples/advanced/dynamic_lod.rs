//! # Advanced Example: Manual LOD Implementation
//!
//! **NOTE**: This is an advanced example showing LOD internals.
//! **For typical usage, see `simple_terrain.rs` or `octree_terrain.rs` instead.**
//!
//! This example demonstrates low-level manual LOD management using `ChunkManager`
//! and custom distance-based region spawning. The high-level `VoxelTerrainPlugin`
//! handles all of this automatically.
//!
//! Demonstrates real-time Level of Detail (LOD) transitions based on camera distance.
//! Large terrain area where chunks spawn/despawn dynamically as you move around.
//!
//! ## LOD Strategy (5 levels with 2× jumps)
//!
//! - **LOD 0** (0-35m): 0.25m voxels, 16 chunks per region (4×4 grid)
//! - **LOD 1** (35-75m): 0.5m voxels, 4 chunks per region (2×2 grid)
//! - **LOD 2** (75-150m): 1.0m voxels, 1 chunk per region
//! - **LOD 3** (150-300m): 2.0m voxels, 1 chunk per region
//! - **LOD 4** (>300m): 4.0m voxels, 1 chunk per region
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
use voxel::{
    Chunk, Voxel, VoxelType, CHUNK_SIZE,
    lod::{LodLevel, LOD_4_DISTANCE, HYSTERESIS_BUFFER, ChunkManager, RegionCoord},
    terrain::terrain_height,
};

// World configuration
const WORLD_SIZE: f32 = 256.0; // 256m × 256m world
const REGION_SIZE: f32 = 32.0; // Each region is 32m × 32m

// Example-specific extension trait for LodLevel colors
trait LodLevelExt {
    fn color(&self) -> Color;
}

impl LodLevelExt for LodLevel {
    fn color(&self) -> Color {
        match self {
            LodLevel::Lod0 => Color::srgb(0.1, 0.8, 0.1),   // Bright green
            LodLevel::Lod1 => Color::srgb(0.3, 0.7, 0.2),   // Green
            LodLevel::Lod2 => Color::srgb(0.2, 0.6, 0.8),   // Blue
            LodLevel::Lod3 => Color::srgb(0.6, 0.4, 0.2),   // Light brown
            LodLevel::Lod4 => Color::srgb(0.7, 0.3, 0.1),   // Dark brown
            LodLevel::None => Color::BLACK,
        }
    }
}

#[derive(Component)]
struct RegionChunks {
    _region: RegionCoord,
    _lod: LodLevel,
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
    let check_radius = ((LOD_4_DISTANCE + HYSTERESIS_BUFFER) / REGION_SIZE).ceil() as i32 + 1;

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
                .get_region(region)
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
    for (region, current_lod, entities) in chunk_manager.regions_with_data() {
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
        chunk_manager.remove_region(region);
    }

    // Spawn new regions or update existing ones
    for (region, desired_lod) in desired_regions {
        if !chunk_manager.has_region(region) {
            // Spawn new region at desired LOD
            let entities = spawn_region(
                &mut commands,
                &mut meshes,
                &mut materials,
                region,
                desired_lod,
            );
            chunk_manager.insert_region(region, desired_lod, entities);
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
                    RegionChunks { _region: region, _lod: lod },
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

/// Debug UI system
fn debug_ui(
    chunk_manager: Res<ChunkManager>,
    camera_query: Query<&Transform, With<Camera3d>>,
) {
    if let Ok(camera_transform) = camera_query.single() {
        let camera_pos = camera_transform.translation;

        let total_chunks: usize = chunk_manager
            .regions_with_data()
            .map(|(_, _, entities)| entities.len())
            .sum();

        let lod0_count = chunk_manager
            .regions_with_data()
            .filter(|(_, lod, _)| **lod == LodLevel::Lod0)
            .count();

        let lod1_count = chunk_manager
            .regions_with_data()
            .filter(|(_, lod, _)| **lod == LodLevel::Lod1)
            .count();

        let lod2_count = chunk_manager
            .regions_with_data()
            .filter(|(_, lod, _)| **lod == LodLevel::Lod2)
            .count();

        let lod3_count = chunk_manager
            .regions_with_data()
            .filter(|(_, lod, _)| **lod == LodLevel::Lod3)
            .count();

        let lod4_count = chunk_manager
            .regions_with_data()
            .filter(|(_, lod, _)| **lod == LodLevel::Lod4)
            .count();

        info!(
            "Pos: ({:.1}, {:.1}, {:.1}) | Regions: {} (L0:{} L1:{} L2:{} L3:{} L4:{}) | Chunks: {}",
            camera_pos.x, camera_pos.y, camera_pos.z,
            chunk_manager.region_count(),
            lod0_count, lod1_count, lod2_count, lod3_count, lod4_count,
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
