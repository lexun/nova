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
