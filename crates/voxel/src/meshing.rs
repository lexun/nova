//! Mesh generation from voxel data
//!
//! Implements greedy meshing algorithm to convert voxel chunks into optimized triangle meshes.

use crate::{Chunk, VoxelType, CHUNK_SIZE};
use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;

/// Get color for a voxel type (RGBA in linear color space)
fn voxel_color(voxel_type: VoxelType) -> [f32; 4] {
    match voxel_type {
        VoxelType::Air => [0.0, 0.0, 0.0, 0.0], // Should never be rendered
        VoxelType::Grass => [0.3, 0.6, 0.2, 1.0], // Green
        VoxelType::Dirt => [0.5, 0.35, 0.2, 1.0], // Brown
        VoxelType::Stone => [0.5, 0.5, 0.5, 1.0], // Gray
    }
}

/// Generate a mesh from a voxel chunk using greedy meshing
pub fn generate_chunk_mesh(chunk: &Chunk, voxel_size: f32) -> Mesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut colors = Vec::new();
    let mut indices = Vec::new();

    // For each axis (X, Y, Z) and direction (positive, negative)
    for axis in 0..3 {
        for direction in [false, true] {
            generate_axis_mesh(
                chunk,
                axis,
                direction,
                voxel_size,
                &mut positions,
                &mut normals,
                &mut uvs,
                &mut colors,
                &mut indices,
            );
        }
    }

    // Create mesh with triangles
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));

    mesh
}

/// Generate mesh for one axis and direction using greedy meshing
fn generate_axis_mesh(
    chunk: &Chunk,
    axis: usize,
    back_face: bool,
    voxel_size: f32,
    positions: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    colors: &mut Vec<[f32; 4]>,
    indices: &mut Vec<u32>,
) {
    // Determine axis permutations
    let (u_axis, v_axis) = match axis {
        0 => (1, 2), // X axis: use Y and Z
        1 => (0, 2), // Y axis: use X and Z
        _ => (0, 1), // Z axis: use X and Y
    };

    // Direction offset
    let offset = if back_face { 0 } else { 1 };

    // Scan through each slice along the axis
    for d in 0..CHUNK_SIZE {
        let mut mask = [[None; CHUNK_SIZE]; CHUNK_SIZE];

        // Build mask for this slice
        for v in 0..CHUNK_SIZE {
            for u in 0..CHUNK_SIZE {
                let mut pos = [0; 3];
                pos[axis] = d;
                pos[u_axis] = u;
                pos[v_axis] = v;

                let voxel = chunk.get_voxel(pos[0], pos[1], pos[2]);

                // Check if we should render a face here
                if let Some(voxel) = voxel {
                    if voxel.voxel_type != VoxelType::Air && voxel.density > 0 {
                        // Check if there's an adjacent voxel blocking this face
                        let mut adj_pos = pos;
                        if back_face {
                            if d > 0 {
                                adj_pos[axis] = d - 1;
                            } else {
                                // Edge of chunk, render face
                                mask[v][u] = Some(voxel.voxel_type);
                                continue;
                            }
                        } else {
                            if d < CHUNK_SIZE - 1 {
                                adj_pos[axis] = d + 1;
                            } else {
                                // Edge of chunk, render face
                                mask[v][u] = Some(voxel.voxel_type);
                                continue;
                            }
                        }

                        let adjacent = chunk.get_voxel(adj_pos[0], adj_pos[1], adj_pos[2]);
                        let is_face_visible = adjacent
                            .map(|v| v.voxel_type == VoxelType::Air || v.density == 0)
                            .unwrap_or(true);

                        if is_face_visible {
                            mask[v][u] = Some(voxel.voxel_type);
                        }
                    }
                }
            }
        }

        // Greedy meshing: merge adjacent faces of same type
        for v in 0..CHUNK_SIZE {
            for u in 0..CHUNK_SIZE {
                if let Some(voxel_type) = mask[v][u] {
                    // Found a face, expand it greedily

                    // Expand in u direction
                    let mut width = 1;
                    while u + width < CHUNK_SIZE && mask[v][u + width] == Some(voxel_type) {
                        width += 1;
                    }

                    // Expand in v direction
                    let mut height = 1;
                    'outer: while v + height < CHUNK_SIZE {
                        for du in 0..width {
                            if mask[v + height][u + du] != Some(voxel_type) {
                                break 'outer;
                            }
                        }
                        height += 1;
                    }

                    // Clear the mask for this rectangle
                    for dv in 0..height {
                        for du in 0..width {
                            mask[v + dv][u + du] = None;
                        }
                    }

                    // Generate quad for this rectangle
                    let d_pos = (d as f32 + offset as f32) * voxel_size;
                    let u_pos = u as f32 * voxel_size;
                    let v_pos = v as f32 * voxel_size;
                    let u_size = width as f32 * voxel_size;
                    let v_size = height as f32 * voxel_size;

                    add_quad(
                        axis,
                        u_axis,
                        v_axis,
                        back_face,
                        d_pos,
                        u_pos,
                        v_pos,
                        u_size,
                        v_size,
                        voxel_size,
                        voxel_type,
                        positions,
                        normals,
                        uvs,
                        colors,
                        indices,
                    );
                }
            }
        }
    }
}

/// Add a quad to the mesh
#[allow(clippy::too_many_arguments)]
fn add_quad(
    axis: usize,
    u_axis: usize,
    v_axis: usize,
    back_face: bool,
    d_pos: f32,
    u_pos: f32,
    v_pos: f32,
    u_size: f32,
    v_size: f32,
    _voxel_size: f32,
    _voxel_type: VoxelType,
    positions: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    colors: &mut Vec<[f32; 4]>,
    indices: &mut Vec<u32>,
) {
    let base_index = positions.len() as u32;

    // Calculate the 4 corners of the quad
    let mut corners = [[0.0; 3]; 4];
    for corner in &mut corners {
        corner[axis] = d_pos;
    }

    corners[0][u_axis] = u_pos;
    corners[0][v_axis] = v_pos;

    corners[1][u_axis] = u_pos + u_size;
    corners[1][v_axis] = v_pos;

    corners[2][u_axis] = u_pos + u_size;
    corners[2][v_axis] = v_pos + v_size;

    corners[3][u_axis] = u_pos;
    corners[3][v_axis] = v_pos + v_size;

    // Calculate normal
    let mut normal = [0.0; 3];
    normal[axis] = if back_face { -1.0 } else { 1.0 };

    // Get color for this voxel type
    let color = voxel_color(_voxel_type);

    // Add vertices
    for corner in &corners {
        positions.push(*corner);
        normals.push(normal);
        colors.push(color);
    }

    // Add UVs - texture tiles based on quad size in voxel units
    // Each material occupies 0.25 of atlas width (4 materials total)
    const ATLAS_REGION_WIDTH: f32 = 0.25;
    let atlas_u_start = get_atlas_u_start(_voxel_type);

    // Calculate UVs based on quad size (in voxel units)
    // Each voxel face should show one complete texture tile
    // Greedy-meshed quads show multiple tiles (e.g., 4×1 voxels = 4×1 texture tiles)
    let u_voxels = u_size / _voxel_size;  // How many voxels wide
    let v_voxels = v_size / _voxel_size;  // How many voxels tall

    // UV coordinates for the 4 corners
    // We want the texture to tile once per voxel
    let corner_uvs = [
        [0.0, 0.0],           // Bottom-left
        [u_voxels, 0.0],      // Bottom-right
        [u_voxels, v_voxels], // Top-right
        [0.0, v_voxels],      // Top-left
    ];

    for (i, _corner) in corners.iter().enumerate() {
        let u_uv = corner_uvs[i][0];
        let v_uv = corner_uvs[i][1];

        // Direct UV mapping - GPU Repeat mode handles tiling
        // TODO: For atlas support, scale U by ATLAS_REGION_WIDTH and add atlas_u_start
        // TODO: Fix UV orientation inconsistency between faces (some are mirrored)
        uvs.push([u_uv, v_uv]);
    }

    // Add indices (two triangles)
    // Y-axis faces need inverted winding order due to axis orientation
    let needs_flip = (axis == 1) ^ back_face;

    if needs_flip {
        // Flip winding order
        indices.extend_from_slice(&[
            base_index,
            base_index + 2,
            base_index + 1,
            base_index,
            base_index + 3,
            base_index + 2,
        ]);
    } else {
        indices.extend_from_slice(&[
            base_index,
            base_index + 1,
            base_index + 2,
            base_index,
            base_index + 2,
            base_index + 3,
        ]);
    }
}

/// Get the starting U coordinate for a voxel type's region in the texture atlas
/// Atlas layout: [Air | Grass | Dirt | Stone] each 0.25 wide
fn get_atlas_u_start(voxel_type: VoxelType) -> f32 {
    match voxel_type {
        VoxelType::Air => 0.0,
        VoxelType::Grass => 0.25,
        VoxelType::Dirt => 0.5,
        VoxelType::Stone => 0.75,
    }
}

