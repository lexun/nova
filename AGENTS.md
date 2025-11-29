## Issue Tracking with bd (beads)

**IMPORTANT**: This project uses **bd (beads)** for ALL issue tracking. Do NOT use markdown TODOs, task lists, or other tracking methods.

### Why bd?

- Dependency-aware: Track blockers and relationships between issues
- Git-friendly: Auto-syncs to JSONL for version control
- Agent-optimized: JSON output, ready work detection, discovered-from links
- Prevents duplicate tracking systems and confusion

### Quick Start

**Check for ready work:**
```bash
bd ready --json
```

**Create new issues:**
```bash
bd create "Issue title" -t bug|feature|task -p 0-4 --json
bd create "Issue title" -p 1 --deps discovered-from:bd-123 --json
```

**Claim and update:**
```bash
bd update bd-42 --status in_progress --json
bd update bd-42 --priority 1 --json
```

**Complete work:**
```bash
bd close bd-42 --reason "Completed" --json
```

### Issue Types

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

### Workflow for AI Agents

1. **Check ready work**: `bd ready` shows unblocked issues
2. **Claim your task**: `bd update <id> --status in_progress`
3. **Work on it**: Implement, test, document
4. **Discover new work?** Create linked issue:
   - `bd create "Found bug" -p 1 --deps discovered-from:<parent-id>`
5. **Complete**: `bd close <id> --reason "Done"`
6. **Commit together**: Always commit the `.beads/issues.jsonl` file together with the code changes so issue state stays in sync with code state

### Auto-Sync

bd automatically syncs with git:
- Exports to `.beads/issues.jsonl` after changes (5s debounce)
- Imports from JSONL when newer (e.g., after `git pull`)
- No manual export/import needed!

### MCP Server (Recommended)

If using Claude or MCP-compatible clients, install the beads MCP server:

```bash
pip install beads-mcp
```

Add to MCP config (e.g., `~/.config/claude/config.json`):
```json
{
  "beads": {
    "command": "beads-mcp",
    "args": []
  }
}
```

Then use `mcp__beads__*` functions instead of CLI commands.

### Planning and Exploration

**Use bd (beads) for ALL planning and exploration work.**

When exploring designs, architectures, or solutions:

1. **Create issues in bd** - Use issue notes for detailed analysis (markdown supported)
2. **Use epics and dependencies** - Structure multi-part explorations
3. **Close when complete** - Knowledge captured in closed issues

**DO NOT create planning documents** unless absolutely necessary for immediate collaboration.

If a temporary document is unavoidable:
- Use `/tmp/nova-planning/` (NOT tracked in git)
- Delete immediately after capturing in bd
- Never commit planning documents

**Why bd-only?**
- ✅ Prevents duplicate tracking systems
- ✅ Git history shows what was decided AND when
- ✅ Dependencies show decision relationships
- ✅ Searchable via `bd list` and `bd show`
- ✅ Never gets out of sync with code
- ✅ Clean repository (no orphaned docs)

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

- ✅ Use bd for ALL task tracking AND planning
- ✅ Always use `--json` flag for programmatic use
- ✅ Link discovered work with `discovered-from` dependencies
- ✅ Check `bd ready` before asking "what should I work on?"
- ✅ Use issue notes for detailed analysis/exploration
- ✅ Follow git commit message guidelines (imperative, <50 chars)
- ❌ Do NOT create markdown TODO lists
- ❌ Do NOT create planning documents (use bd issues)
- ❌ Do NOT use external issue trackers
- ❌ Do NOT duplicate tracking systems

For more details, see README.md and QUICKSTART.md.

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
