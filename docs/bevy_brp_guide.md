# Bevy Remote Protocol (BRP) Guide

## Overview

bevy_brp enables autonomous AI development and testing through a JSON-RPC HTTP server (port 15702). Think of it as giving AI agents the ability to interact with your running Bevy game in real-time, similar to how Cursor 2.0's built-in browser works for web development.

## What's Enabled

- **Real-time entity/component manipulation** - Query, create, modify, and delete entities and components
- **Visual verification** - Take screenshots to verify rendering and visual state
- **Input simulation** - Send keyboard/mouse input to test interactions
- **Window control** - Manage window state and focus
- **27+ specialized tools** via the MCP server

## When to Use BRP

### Development Iteration
- Rapidly prototype new features without recompiling
- Test different component values to find the right parameters
- Iterate on visual appearance (colors, sizes, positions)

### Visual Verification
- Capture screenshots to verify rendering output
- Compare before/after states when making changes
- Debug visual issues that are hard to describe in text

### Component Testing
- Test entity/component relationships
- Verify that components are being added/removed correctly
- Query game state to understand what's happening

### Current Use Cases in Nova

1. **Voxel System Development**
   - Query voxel entities to verify chunk generation
   - Screenshot voxel visualization to check scale and appearance
   - Modify voxel colors/materials without recompiling

2. **Camera System Testing**
   - Query camera transform to verify position
   - Test camera movement by injecting keyboard input
   - Verify cursor grab state

3. **Scene Debugging**
   - Count entities to verify chunk loading
   - Check material properties
   - Inspect transform hierarchies

## Common Workflows

### Workflow 1: Develop New Voxel Feature

```
1. Start Nova with BRP enabled (already configured in example/main.rs)
2. Use BRP tools to:
   - Query existing voxel entities
   - Take screenshot of current state
   - Add new voxel entities with test data
   - Take screenshot to compare
   - Iterate on parameters
3. Once satisfied, port the changes to Rust code
```

### Workflow 2: Debug Rendering Issue

```
1. Take screenshot showing the problem
2. Query entities to understand scene state
3. Modify component values to isolate the issue
4. Take screenshot to verify fix
5. Document findings and implement permanent fix
```

### Workflow 3: Test Camera System

```
1. Query camera entity and transform component
2. Send keyboard input (W/A/S/D) via BRP
3. Query camera transform again to verify movement
4. Screenshot to verify visual state
5. Test edge cases (cursor grab, window focus)
```

## Tool Categories

### Core BRP Tools (via RemoteHttpPlugin)

**Entity Management**
- `bevy/get` - Get all entities and components
- `bevy/query` - Query entities with specific components
- `bevy/spawn` - Spawn new entities
- `bevy/insert` - Add components to entities
- `bevy/remove` - Remove components from entities
- `bevy/destroy` - Destroy entities
- `bevy/reparent` - Change entity parent

**Type Introspection**
- `bevy/list` - List all registered component types

### Extended Tools (via BrpExtrasPlugin)

**Screenshots**
- Take screenshots of current viewport
- Compare visual states
- Document rendering issues

**Keyboard Input**
- Send key press/release events
- Simulate user input
- Test input handling

**Window Control**
- Manage window focus
- Control window state
- Test window-related features

## Integration with Current Nova Code

The example already has BRP configured:

```rust
// In crates/example/src/main.rs
use bevy_remote::http::RemoteHttpPlugin;
use bevy_brp_extras::BrpExtrasPlugin;

fn main() {
    let mut app = engine::create_app();

    // MCP Server Integration - Enable autonomous AI development and testing
    app.add_plugins(RemoteHttpPlugin::default());  // Bevy Remote Protocol (port 15702)
    app.add_plugins(BrpExtrasPlugin);  // Screenshots, keyboard input, window control

    // ... rest of setup
}
```

When you run the example, BRP automatically starts listening on `http://localhost:15702`.

## Validation Tests

Once the MCP servers are installed, run these tests to verify BRP is working:

1. **Basic connectivity** - Query all entities
2. **Screenshot capability** - Take a screenshot of the voxel scene
3. **Entity query** - Find all entities with Camera3d component
4. **Input simulation** - Send 'W' key press to test camera movement
5. **Component inspection** - Query Transform component on camera entity

## Next Steps

1. Install MCP servers: `install-mcp-servers`
2. Restart Claude Code conversation to load new tools
3. Run the 5 validation tests above
4. Try iterating on voxel visualization using BRP tools
5. Document any issues or improvements needed

## References

- Bevy Remote Protocol: https://github.com/bevyengine/bevy/tree/main/crates/bevy_remote
- bevy_brp_extras: https://crates.io/crates/bevy_brp_extras
- bevy_brp_mcp: MCP server wrapping BRP with 27+ specialized tools
