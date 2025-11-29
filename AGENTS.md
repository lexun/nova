## Issue Tracking with Memex

**IMPORTANT**: This project uses **Memex** for ALL issue tracking via MCP. Do NOT use markdown TODOs, task lists, or other tracking methods.

### Why Memex?

- **MCP-native**: Integrated directly via Model Context Protocol
- **Dependency-aware**: Track blockers and relationships between tasks
- **Agent-optimized**: JSON output, structured queries, ready work detection
- **Persistent**: SQLite backend stores task history
- **Project-scoped**: All nova tasks use `project: "nova"`

### Quick Start

**List tasks:**
```
mcp__memex__list_tasks(project: "nova")
mcp__memex__ready_tasks(project: "nova")  // Show unblocked work
```

**Create tasks:**
```
mcp__memex__create_task(
  title: "Fix bug in meshing",
  project: "nova",
  task_type: "bug",
  priority: 1,
  description: "Detailed description..."
)
```

**Update tasks:**
```
mcp__memex__update_task(id: "abc123", status: "in_progress")
mcp__memex__update_task(id: "abc123", priority: 0)
```

**Complete tasks:**
```
mcp__memex__close_task(id: "abc123", reason: "Fixed in commit abc123")
```

### Task Types

- `bug` - Something broken
- `feature` - New functionality
- `task` - Work item (tests, docs, refactoring)
- `epic` - Large feature with subtasks
- `chore` - Maintenance (dependencies, tooling)

### Priorities

- `0` - Critical (security, data loss, broken builds)
- `1` - High (major features, important bugs)
- `2` - Medium (default, nice-to-have)
- `3` - Low (polish, optimization)
- `4` - Backlog (future ideas)

### Task Dependencies

Track relationships between tasks:
```
mcp__memex__add_dependency(
  from_task_id: "abc123",
  to_task_id: "def456",
  relation_type: "blocks"  // or "depends_on", "relates_to"
)
```

### Workflow for AI Agents

1. **Check ready work**: `mcp__memex__ready_tasks(project: "nova")`
2. **Claim your task**: `mcp__memex__update_task(id, status: "in_progress")`
3. **Work on it**: Implement, test, document
4. **Add updates**: `mcp__memex__add_task_update(task_id, content: "Progress note")`
5. **Complete**: `mcp__memex__close_task(id, reason: "Completed")`

### Planning and Exploration

**Use Memex for planning and exploration work, and create documentation files to capture knowledge.**

When exploring designs, architectures, or solutions:

1. **Create tasks in Memex** - Use task updates for progress notes
2. **Use dependencies** - Structure multi-part explorations
3. **Document in `docs/` folders** - Capture technical decisions and architecture
   - Use lowercase filenames (e.g., `uv_mapping.md`, `lod_system.md`)
   - Place in appropriate crate: `crates/voxel/docs/`, `crates/engine/docs/`
4. **Close tasks when complete** - Reference documentation in close reason

**Documentation Guidelines:**
- ✅ Create docs for complex technical decisions
- ✅ Use Markdown with clear structure
- ✅ Include code examples where relevant
- ✅ Keep docs close to the code (in crate docs/ folders)
- ✅ Lowercase filenames with underscores
- ❌ Do NOT create docs in project root
- ❌ Do NOT create planning docs in `/tmp/`

**Existing Documentation:**
- `crates/voxel/docs/uv_mapping.md` - UV mapping system
- `crates/voxel/docs/lod_system.md` - LOD strategies and octree
- `crates/voxel/docs/spherical_terrain.md` - Planet rendering approaches

### Git Commit Messages

Follow these guidelines for all commit messages:

**Format:**
- **Title only** - No body, no additional content
- **Imperative mood** - "Add feature" not "Added feature" or "Adds feature"
- **Capitalize first letter** - "Fix bug" not "fix bug"
- **No period at end** - "Update docs" not "Update docs."
- **Under 50 characters** - Keep it concise and scannable

**Examples:**
- ✅ `Add user authentication`
- ✅ `Fix memory leak in worker pool`
- ✅ `Update dependencies to latest versions`
- ❌ `added user authentication` (not imperative, not capitalized)
- ❌ `Fixes the memory leak in the worker pool that was causing issues` (too long)
- ❌ `Update docs.` (has period)
- ❌ Any commit with body text or multiple lines

**Critical**: Use `git commit -m "Title"` NOT `git commit -m "$(cat <<EOF ...)"` with heredoc.
The title is the ONLY content. No attribution, no body, no Co-Authored-By, no emojis.

**Rationale:**
Short, imperative commits create a clean, scannable git history. Each commit should represent a single logical change that can be described in one concise line.

### Important Rules

- ✅ Use Memex (via MCP) for ALL task tracking
- ✅ Always use `project: "nova"` parameter
- ✅ Create docs in crate `docs/` folders for technical decisions
- ✅ Check `ready_tasks()` before asking "what should I work on?"
- ✅ Use task updates for progress notes
- ✅ Follow git commit message guidelines (imperative, <50 chars)
- ❌ Do NOT create markdown TODO lists
- ❌ Do NOT use external issue trackers
- ❌ Do NOT duplicate tracking systems

## Visual Verification Workflow for 3D Graphics

**IMPORTANT**: When debugging 3D rendering issues (UV mapping, textures, orientation, etc.), use this systematic verification approach.

### Why Systematic Visual Verification?

AI vision models struggle with complex 3D scenes:
- ❌ Multiple objects create visual ambiguity
- ❌ Perspective distortion warps appearances
- ❌ Oblique viewing angles make orientation unclear
- ❌ Cannot reliably identify which face is which

**Solution**: Systematic single-object perpendicular verification

### Verification Workflow

**1. Create Isolated Test Scene**
- Single simple object (e.g., 1×1×1 cube)
- Clear directional texture (e.g., letter "F", arrow, asymmetric pattern)
- Positioned at known coordinates (e.g., origin)

**2. Implement Perpendicular Camera Controls**
- Keyboard shortcuts to view each face directly (perpendicular)
- Document exact camera positions for each view
- Console output showing which face is being viewed

**3. Capture Screenshots Systematically**
- One screenshot per face, looking directly at it
- Save with descriptive names (e.g., `face_X+.png`, `face_Y-.png`)
- Document camera position and expected vs actual appearance

**4. Analyze Each Face Individually**
- Compare actual texture orientation to expected
- Look for mirroring (horizontal/vertical flips)
- Look for rotation issues
- Document findings in table format

**5. Verify Every Fix**
- After making code changes, re-test ALL faces
- Don't assume fixing one face didn't break others
- Capture new screenshots with `_fixed` suffix
- Compare before/after results

### Example: UV Orientation Testing

**Test object**: 1×1×1 cube with "F" texture
- F pattern: vertical stroke on LEFT, horizontal strokes extend RIGHT
- Only ONE correct upright orientation
- Easy to spot mirroring/rotation errors

**Camera positions for cube at origin**:
```rust
// Distance from origin for perpendicular view
let distance = 3.0;

X+ (right):  Transform::from_xyz(distance, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y)
X- (left):   Transform::from_xyz(-distance, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y)
Y+ (top):    Transform::from_xyz(0.0, distance, 0.0).looking_at(Vec3::ZERO, Vec3::Z)
Y- (bottom): Transform::from_xyz(0.0, -distance, 0.0).looking_at(Vec3::ZERO, Vec3::Z)
Z+ (front):  Transform::from_xyz(0.0, 0.0, distance).looking_at(Vec3::ZERO, Vec3::Y)
Z- (back):   Transform::from_xyz(0.0, 0.0, -distance).looking_at(Vec3::ZERO, Vec3::Y)
```

**Analysis table**:
| Face | Camera Pos | Expected | Actual | Status |
|------|-----------|----------|--------|--------|
| Z+ | (0,0,3) | F upright | F upright | ✓ |
| X+ | (3,0,0) | F upright | F mirrored-H | ❌ |
| ... | ... | ... | ... | ... |

### Tools to Use

**BRP (Bevy Remote Protocol)** for automated testing:
```bash
# Launch app with BRP enabled
cargo run --example uv_debug_single_face --release

# Send keyboard input to view specific face
mcp__bevy_brp__brp_extras_send_keys(keys=["Digit1"])  # View X+ face

# Capture screenshot
mcp__bevy_brp__brp_extras_screenshot(path="/tmp/face_X+.png")
```

### Best Practices

✅ **Always verify fixes** - Test all faces after any change
✅ **Use directional textures** - Makes orientation obvious (e.g., letter "F")
✅ **Document camera positions** - Correlate screenshots with viewpoint
✅ **One object at a time** - Eliminates visual ambiguity
✅ **Perpendicular views only** - No perspective distortion
✅ **Systematic approach** - Same process every time
✅ **Proper camera framing** - Center object in viewport, ensure full visibility
✅ **Collaborate on visual analysis** - Share screenshots with user to verify interpretation
✅ **Check both axes** - Look for horizontal AND vertical orientation issues

❌ **Don't trust complex scenes** - Multiple objects cause confusion
❌ **Don't use oblique angles** - Perspective makes analysis unreliable
❌ **Don't skip verification** - Fixes may have unintended side effects
❌ **Don't use symmetric textures** - Can't detect orientation issues
❌ **Don't assume fix is complete** - May have fixed one axis but not another
❌ **Don't trust clipped views** - Poor framing can hide critical visual information

### When to Use This Workflow

Use systematic visual verification when working on:
- UV mapping and texture coordinates
- Face orientation and winding order
- Texture atlas mapping
- Normal calculations
- Any visual rendering issue where "it looks wrong" is the symptom

The time investment in rigorous verification pays off by:
- Catching regressions immediately
- Building confidence in fixes
- Enabling autonomous iteration without user verification
- Scaling to more complex objects and scenarios

### Critical Learnings from UV Orientation Debugging

**Issue 1: Camera Framing**
- **Problem**: Default camera at origin (0,0,3) looking at origin (0,0,0) resulted in cube being off-center and partially clipped
- **Root Cause**: 1×1×1 voxel cube extends from (0,0,0) to (1,1,1), so center is at (0.5, 0.5, 0.5)
- **Solution**: Position camera to look at cube_center `Vec3::new(0.5, 0.5, 0.5)` and adjust camera position accordingly
- **Lesson**: Always calculate and target the actual center of the object being viewed

**Issue 2: Vertical UV Flip**
- **Problem**: Textures appeared upside down (letter "F" had long bar at bottom instead of top)
- **Root Cause**: Mismatch between image storage (row 0 = top) and UV convention (V=0 = bottom)
- **Solution**: Flip V coordinates in corner_uvs array: `[0.0, v_voxels]` instead of `[0.0, 0.0]`
- **Lesson**: When working with textures, verify BOTH horizontal AND vertical orientation

**Issue 3: False Confidence from Incomplete Testing**
- **Problem**: Initial "fix" only addressed horizontal mirroring on X-axis faces, missed vertical flip on ALL faces
- **Root Cause**: Failed to properly analyze screenshots due to poor framing and incomplete visual parsing
- **Solution**: Collaborate with user on visual analysis - share screenshots and compare interpretations
- **Lesson**: Never claim a fix is complete without user verification of actual visual output

**AI Vision Limitations with 3D Graphics:**
1. Cannot reliably parse clipped/partial views of objects
2. May confuse horizontal and vertical orientation issues
3. Can misidentify which parts of asymmetric patterns are "top" vs "bottom"
4. Requires explicit collaboration with human to verify visual correctness

**Best Practice Going Forward:**
After implementing any graphics fix, capture a screenshot and explicitly ask user:
"I've captured a screenshot at `/tmp/filename.png`. My interpretation is [describe what I see]. Does this match what you're seeing? Is the orientation correct?"
