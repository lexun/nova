//! Integration tests for TerrainGenerator trait and implementations

use bevy::math::Vec3;
use voxel::terrain::{
    DebugPatternGenerator, DefaultTerrainGenerator, FlatTerrainGenerator, TerrainGenerator,
};
use voxel::{VoxelType, CHUNK_SIZE};

#[test]
fn test_default_terrain_generator() {
    let generator = DefaultTerrainGenerator::new();
    let chunk = generator.generate_chunk(Vec3::ZERO, 0.25);

    // Should have some voxels set
    let mut voxel_count = 0;
    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                if let Some(voxel) = chunk.get_voxel(x, y, z) {
                    if voxel.voxel_type != VoxelType::Air {
                        voxel_count += 1;
                    }
                }
            }
        }
    }

    assert!(voxel_count > 0, "Default generator should create terrain");
}

#[test]
fn test_default_generator_has_layered_materials() {
    let generator = DefaultTerrainGenerator::new();
    // Use position with deeper terrain
    let chunk = generator.generate_chunk(Vec3::ZERO, 0.25);

    // Should have grass, dirt, and stone
    let mut has_grass = false;
    let mut has_dirt = false;
    let mut has_stone = false;

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                if let Some(voxel) = chunk.get_voxel(x, y, z) {
                    match voxel.voxel_type {
                        VoxelType::Grass => has_grass = true,
                        VoxelType::Dirt => has_dirt = true,
                        VoxelType::Stone => has_stone = true,
                        _ => {}
                    }
                }
            }
        }
    }

    assert!(has_grass, "Should have grass layer");
    assert!(has_dirt, "Should have dirt layer");
    assert!(has_stone, "Should have stone foundation");
}

#[test]
fn test_flat_terrain_generator() {
    let generator = FlatTerrainGenerator::new(5.0); // 5 meters high
    let chunk = generator.generate_chunk(Vec3::ZERO, 0.25);

    // Calculate expected height in voxels: 5.0 / 0.25 = 20 voxels
    let expected_height = 20;

    // Check a column - should be solid up to height, empty above
    for y in 0..CHUNK_SIZE {
        if let Some(voxel) = chunk.get_voxel(10, y, 10) {
            if y <= expected_height {
                assert_eq!(
                    voxel.voxel_type,
                    VoxelType::Grass,
                    "Voxel at y={} should be grass",
                    y
                );
            } else {
                assert_eq!(
                    voxel.voxel_type,
                    VoxelType::Air,
                    "Voxel at y={} should be air",
                    y
                );
            }
        }
    }
}

#[test]
fn test_flat_terrain_with_custom_voxel_type() {
    let generator = FlatTerrainGenerator::new(2.0).with_voxel_type(VoxelType::Stone);
    let chunk = generator.generate_chunk(Vec3::ZERO, 1.0);

    // 2.0 meters / 1.0 voxel_size = 2 voxels height
    let voxel = chunk.get_voxel(0, 1, 0).unwrap();
    assert_eq!(voxel.voxel_type, VoxelType::Stone);

    let voxel_above = chunk.get_voxel(0, 3, 0).unwrap();
    assert_eq!(voxel_above.voxel_type, VoxelType::Air);
}

#[test]
fn test_debug_checkerboard_generator() {
    let generator = DebugPatternGenerator::checkerboard(3.0);
    let chunk = generator.generate_chunk(Vec3::ZERO, 0.5);

    // Should have alternating stone and grass
    let mut has_stone = false;
    let mut has_grass = false;

    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            if let Some(voxel) = chunk.get_voxel(x, 0, z) {
                match voxel.voxel_type {
                    VoxelType::Stone => has_stone = true,
                    VoxelType::Grass => has_grass = true,
                    _ => {}
                }
            }
        }
    }

    assert!(has_stone && has_grass, "Checkerboard should have both materials");
}

#[test]
fn test_debug_stripes_generator() {
    let generator = DebugPatternGenerator::stripes(3.0);
    let chunk = generator.generate_chunk(Vec3::ZERO, 0.5);

    // Should have stone and dirt stripes
    let mut has_stone = false;
    let mut has_dirt = false;

    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            if let Some(voxel) = chunk.get_voxel(x, 0, z) {
                match voxel.voxel_type {
                    VoxelType::Stone => has_stone = true,
                    VoxelType::Dirt => has_dirt = true,
                    _ => {}
                }
            }
        }
    }

    assert!(has_stone && has_dirt, "Stripes should have both materials");
}

#[test]
fn test_terrain_generator_is_scale_independent() {
    let generator = DefaultTerrainGenerator::new();

    // Generate same world position at different voxel sizes
    let world_pos = Vec3::new(50.0, 0.0, 50.0);
    let chunk_fine = generator.generate_chunk(world_pos, 0.25);
    let chunk_coarse = generator.generate_chunk(world_pos, 1.0);

    // Both should have terrain (not empty)
    let fine_has_voxels = chunk_fine.get_voxel(0, 5, 0).unwrap().voxel_type != VoxelType::Air;
    let coarse_has_voxels = chunk_coarse.get_voxel(0, 1, 0).unwrap().voxel_type != VoxelType::Air;

    assert!(fine_has_voxels && coarse_has_voxels, "Terrain should exist at both scales");
}

#[test]
fn test_default_generator_with_max_world_size() {
    let generator = DefaultTerrainGenerator::new().with_max_world_size(4.0);
    let chunk = generator.generate_chunk(Vec3::ZERO, 0.25);

    // 4m / 0.25 = 16 voxels should be the limit
    // Voxels beyond should be air
    let _within_bounds = chunk.get_voxel(15, 5, 15);
    let outside_bounds = chunk.get_voxel(17, 5, 17);

    // Within bounds should potentially have terrain
    // Outside bounds should definitely be air
    assert_eq!(
        outside_bounds.unwrap().voxel_type,
        VoxelType::Air,
        "Voxels outside max_world_size should be air"
    );
}

#[test]
fn test_terrain_generator_trait_object() {
    // Test that we can use TerrainGenerator as a trait object
    let generators: Vec<Box<dyn TerrainGenerator>> = vec![
        Box::new(DefaultTerrainGenerator::new()),
        Box::new(FlatTerrainGenerator::new(5.0)),
        Box::new(DebugPatternGenerator::checkerboard(3.0)),
    ];

    for generator in generators {
        let chunk = generator.generate_chunk(Vec3::ZERO, 0.5);
        // Just verify it doesn't panic and returns a chunk
        let _voxel = chunk.get_voxel(0, 0, 0);
    }
}
