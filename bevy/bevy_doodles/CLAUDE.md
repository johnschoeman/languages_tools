# Claude Instructions for Bevy Doodles Project

## Bevy Version

This project uses **Bevy 0.18** (released January 2026).

## Documentation Resources

When working on this project, always reference the current Bevy 0.18 documentation:

- **Official Getting Started**: https://bevy.org/learn/quick-start/getting-started/
- **API Documentation**: https://docs.rs/bevy/0.18.0/
- **Release Notes**: https://bevy.org/news/bevy-0-18/
- **Unofficial Bevy Cheat Book**: https://bevy-cheatbook.github.io/

## Important Notes

- Always check the Bevy 0.18 documentation when implementing features or suggesting code patterns
- Bevy 0.18 introduced cargo feature collections for scenario-driven features (2D, 3D, UI)
- The project uses the standard `DefaultPlugins` setup
- When searching for Bevy documentation, include "Bevy 0.18" and the current year (2026) in queries

## Coding Guidelines

- Use idiomatic Rust whenever appropriate
- Keep documentation concise and avoid being overly verbose

## Git Workflow

User manages all commits. You remind and suggest, never commit.

**After significant work:**
1. Update `.claude/SESSION_LOG.md` with session context
2. Run `git status` and `git log --oneline -5` to check state
3. Write commit message to `commit-msg.txt` (always first)
4. Display message and remind: "You may want to commit these changes"

**Format:** Imperative mood, 50 char title, explain why/what in body.
**Skip for:** Trivial changes. **Never:** Run git commands or skip temp file.

Use `/suggest-commit` to generate a commit message interactively.

## Development Environment

This project uses [devenv](https://devenv.sh) for reproducible development environments. Configuration lives in `devenv.nix` (shell config) and `devenv.yaml` (nixpkgs input). Devenv provides:

- System libraries: udev, alsa-lib, vulkan-loader, X11, Wayland
- Build tools: pkg-config, cmake
- Rust stable toolchain (managed by devenv)

### Usage

Enter the development environment:
```bash
devenv shell
```

Or use direnv for automatic activation (`.envrc` is already configured).

## Verifying Visual Changes

When making visual changes to the Bevy app, always create a screenshot to verify the output is correct. Claude can view and analyze screenshots.

### Automated Screenshot

Use the automated screenshot script to capture the current state:
```bash
./screenshot.sh
```

This will:
- Build and run the app
- Wait for the scene to render
- Take a screenshot to `./tmp/bevy_screenshot_auto.png`
- Exit automatically

After running the script, use the Read tool to view the screenshot and verify visual changes.

### Manual Screenshot

During normal app operation, press **F12** to save a screenshot to `./tmp/bevy_screenshot_N.png` (where N is an incrementing counter).

## Knowledge Hierarchy

Context for this project lives in layers with different purposes:

- **`CLAUDE.md`** (this file) - Critical instructions, project setup, coding guidelines
- **`.claude/SESSION_LOG.md`** - Session continuity: goals, accomplishments, remaining tasks, current app state
- **`.claude/MEMORY.md`** - Long-term learnings: Bevy API gotchas, architecture patterns, user preferences
- **Git history** - Detailed code changes; commit messages explain the "why"

### Session Start
Read SESSION_LOG.md and MEMORY.md at the start of every session to pick up where things left off.

### Continuous Updates
Keep SESSION_LOG.md and MEMORY.md up to date **as you work**, not just at the end:

- **SESSION_LOG.md**: Update after completing each meaningful task or milestone. Add new session headers at the start of each session. Keep "Current App State" reflecting reality. Move completed items out of "Remaining/Deferred" promptly.
- **MEMORY.md**: Update immediately when you discover a new Bevy API gotcha, hit a surprising error, establish a new pattern, or learn a user preference. Remove or correct entries that turn out to be wrong.

Don't batch updates to the end of a session — context can be lost if a session ends unexpectedly.

## Project Structure

- `src/main.rs` - App entry point, plugin/resource/system registration
- `src/scene.rs` - 3D scene setup (cubes, light, ground, camera), rotation systems, color animation
- `src/ui.rs` - UI layout (buttons, input panels), button interaction handling, UI visibility toggle
- `src/text_input.rs` - Text input focus management, keyboard input handling, cursor blink
- `src/debug.rs` - Debug mode overlay, gizmo axes, screenshot systems (manual + auto)
- `Cargo.toml` - Dependencies (Bevy 0.18)
- `devenv.nix` - Devenv shell configuration (Rust toolchain, system libraries)
- `devenv.yaml` - Devenv inputs (nixpkgs source)
- `screenshot.sh` - Automated screenshot capture script
