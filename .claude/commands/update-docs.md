# Update Documentation

Review recent code changes and update project documentation accordingly.

## Instructions

1. **Identify what changed** - Look at recently modified files, new patterns, or refactored code. Use git status/diff if available, or ask the user what was just completed.

2. **Check if docs need updates** - For each change, consider:
   - New patterns or gotchas that should be documented?
   - Existing docs that are now outdated or incorrect?
   - New files/modules that need to be referenced?

3. **Update the appropriate docs** based on what changed:

   | Change Type | Update These Docs |
   |-------------|-------------------|
   | New GPUI patterns/gotchas | `docs/gpui-guide.md` |
   | Theme/color changes | `docs/gpui-theming.md` or `docs/component-theming-rules.md` |
   | macOS/menu/window changes | `docs/macos-app-guide.md` |
   | New Zed reference paths | `docs/zed-reference.md` |
   | Build/project structure | `CLAUDE.md` |
   | PostCommander UI | `docs/postcommander-ui-specification.md` |

4. **Keep CLAUDE.md slim** - Only add to CLAUDE.md if it's:
   - A new build command
   - A change to project structure
   - A new doc that needs to be in the index

   Everything else goes in the specific docs.

5. **Document gotchas immediately** - If you encountered a tricky bug, confusing API, or non-obvious behavior, add it to the relevant doc so future agents don't hit the same issue.

## Output

After updating, summarize:
- Which docs were updated
- What was added/changed
- Any docs that should be created but weren't (suggest for user approval)
