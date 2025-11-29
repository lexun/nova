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
    // Calculate UVs based on quad size (in voxel units)
    // Each voxel face should show one complete texture tile
    // Greedy-meshed quads show multiple tiles (e.g., 4×1 voxels = 4×1 texture tiles)

    // For X-axis faces (axis==0), u_axis maps to Y (vertical) and v_axis maps to Z (horizontal)
    // We need to swap them so UV texture space aligns correctly
    let (texture_u_voxels, texture_v_voxels) = if axis == 0 {
        // X faces: swap because u_axis=Y (vertical in world) but U is horizontal in texture
        (v_size / _voxel_size, u_size / _voxel_size)
    } else {
        // Y and Z faces: no swap needed
        (u_size / _voxel_size, v_size / _voxel_size)
    };

    // UV coordinates for the 4 corners
    // We want the texture to tile once per voxel
    // NOTE: V is flipped because image row 0 is at top, but UV V=0 is at bottom
    let corner_uvs = [
        [0.0, texture_v_voxels],              // Bottom-left (V flipped)
        [texture_u_voxels, texture_v_voxels], // Bottom-right (V flipped)
        [texture_u_voxels, 0.0],              // Top-right (V flipped)
        [0.0, 0.0],                           // Top-left (V flipped)
    ];

    // Apply UV orientation correction to ensure all faces have consistent texture direction
    // Without this, some faces show mirrored textures due to different axis mappings
    let (u_corrected, v_corrected) = correct_uv_orientation(axis, back_face, &corner_uvs);


    for i in 0..4 {
        // Direct UV mapping - GPU Repeat mode handles tiling
        // TODO: For atlas support, scale U by ATLAS_REGION_WIDTH and add atlas_u_start offset
        uvs.push([u_corrected[i], v_corrected[i]]);
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
/// TODO: Re-enable when adding atlas support
#[allow(dead_code)]
fn get_atlas_u_start(voxel_type: VoxelType) -> f32 {
    match voxel_type {
        VoxelType::Air => 0.0,
        VoxelType::Grass => 0.25,
        VoxelType::Dirt => 0.5,
        VoxelType::Stone => 0.75,
    }
}

/// Correct UV orientation to ensure consistent texture direction across all faces
///
/// Without correction, faces on different axes show mirrored/rotated textures
/// because they use different 3D axis mappings (X→Y, Y→Z, Z→X, etc.)
///
/// This function ensures all faces show textures in a consistent orientation:
/// - U always increases left-to-right when viewing the face head-on
/// - V always increases bottom-to-top
fn correct_uv_orientation(
    axis: usize,
    back_face: bool,
    corner_uvs: &[[f32; 2]; 4],
) -> ([f32; 4], [f32; 4]) {
    // Corner order: [0]=bottom-left, [1]=bottom-right, [2]=top-right, [3]=top-left

    // Start with original UVs
    let orig_u = [corner_uvs[0][0], corner_uvs[1][0], corner_uvs[2][0], corner_uvs[3][0]];
    let orig_v = [corner_uvs[0][1], corner_uvs[1][1], corner_uvs[2][1], corner_uvs[3][1]];

    // Apply transformations based on face to ensure consistent orientation
    // Horizontal flip: swap indices [0,1,2,3] → [1,0,3,2]
    // Vertical flip: swap indices [0,1,2,3] → [3,2,1,0]

    let (u, v) = match (axis, back_face) {
        (0, false) => {
            // X+ (right face): rotate 90° CCW
            // 90° CCW: [0,1,2,3] → [1,2,3,0]
            ([orig_u[1], orig_u[2], orig_u[3], orig_u[0]],
             [orig_v[1], orig_v[2], orig_v[3], orig_v[0]])
        }
        (0, true) => {
            // X- (left face): rotate 90° CW, then flip vertically
            // 90° CW: [0,1,2,3] → [3,0,1,2], then V-flip: swap top-bottom
            ([orig_u[3], orig_u[0], orig_u[1], orig_u[2]],
             [orig_v[0], orig_v[3], orig_v[2], orig_v[1]])
        }
        (1, false) => {
            // Y+ (top face): flip vertically to be readable from north (initial camera view)
            // Vertical flip: [0,1,2,3] → [3,2,1,0]
            (orig_u, [orig_v[3], orig_v[2], orig_v[1], orig_v[0]])
        }
        (1, true) => {
            // Y- (bottom face): no transformation needed - already correct from north perspective
            (orig_u, orig_v)
        }
        (2, false) => {
            // Z+ (front face): no change (reference)
            (orig_u, orig_v)
        }
        (2, true) => {
            // Z- (back face): flip horizontally
            ([orig_u[1], orig_u[0], orig_u[3], orig_u[2]], orig_v)
        }
        _ => (orig_u, orig_v)
    };

    (u, v)
}

