# Session Log

## Session 1

### Goals
- Build an interactive 3D scene with two intersecting cubes as a Bevy learning project
- Add UI controls for rotation, position, and lighting
- Set up a cross-session knowledge system (CLAUDE.md, SESSION_LOG, MEMORY)

### What Was Accomplished
- **Core scene**: Two cubes (main + leaf child) with parent-child hierarchy, point light with shadows, ground plane, camera
- **UI system**: Button panel (auto-rotate, reset, per-axis rotation), text input panels for main cube rotation, leaf cube config (rotation + position), and light position
- **Text input system**: Click-to-focus numeric inputs with cursor blink, validation (digits, minus, decimal), keyboard handling
- **Debug mode**: Toggle with D key, shows coordinate axes via gizmos, rotation readout (Euler + axis-angle)
- **Color animation**: HSL-based color cycling every 20s with smooth interpolation, complementary background/ground colors
- **Screenshot system**: F12 manual screenshots, `AUTO_SCREENSHOT` env var for headless capture
- **UI toggle**: H key hides/shows all UI panels
- **Keyboard controls**: Space (auto-rotate), R (reset), J/U/K/I/L/O (per-axis rotation)
- **Nix flake**: Reproducible dev environment with all Bevy Linux dependencies
- **Knowledge system**: Created SESSION_LOG.md, MEMORY.md, updated CLAUDE.md

### Current App State
Two intersecting cubes on a ground plane. Colors slowly cycle through hues with complementary background. Left side has auto-rotate/reset buttons and input panels for main rotation, leaf config. Right side has per-axis rotation buttons and light position panel. Debug overlay available with D key.

### Decisions Made
- Parent-child ECS hierarchy for cubes (main rotates, leaf inherits + has own transform)
- Right-hand coordinate system (Z out of screen)
- Modular file structure: `scene.rs`, `ui.rs`, `text_input.rs`, `debug.rs`
- HSL color space for pleasant animation with hue wrap-around handling
- "Leaf" terminology instead of "child" for the attached cube
- Euler angles (XYZ order) for user-facing rotation, quaternions internally

### Remaining/Deferred
- No known deferred tasks from this session

## Session 2

### Goals
- Migrate dev environment from flake.nix to devenv

### What Was Accomplished
- **Devenv migration**: Replaced `flake.nix` + `flake.lock` with `devenv.nix`, `devenv.yaml`, `.envrc`
- Used `devenv init` to bootstrap, then customized config for Bevy dependencies
- `languages.rust.channel = "stable"` replaces manual rust-overlay setup
- Added `rust-overlay` input (required by devenv for `languages.rust.channel`)
- Updated X11 package names from deprecated `xorg.libX*` to toplevel `libx*`
- Disabled cachix (not a trusted Nix user)
- Updated `screenshot.sh` to use `devenv shell -- cargo run`
- Updated `CLAUDE.md` development environment docs and project structure
- Updated `.gitignore` with devenv-specific entries
- Verified: `devenv shell` activates cleanly, `cargo build` succeeds

### Current App State
Same as Session 1 (no app code changes). Dev environment now uses devenv instead of nix flakes.

### Decisions Made
- `cachix.enable = false` to avoid noisy warnings (user not a trusted Nix user)
- Use new toplevel `libx11`/`libxcursor`/`libxi`/`libxrandr` names instead of deprecated `xorg.*`
- Keep `nixpkgs-unstable` as the nixpkgs source (matches original flake)

### Remaining/Deferred
- Cachix could be re-enabled after adding user to trusted-users in NixOS config
