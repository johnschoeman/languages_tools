# Project Memory

## Bevy 0.18 API Notes

- **No `clear_color` on Camera3d**: Use the `ClearColor` resource (`init_resource::<ClearColor>()`) to set/change background color
- **Screenshot API**: `Screenshot::primary_window()` spawned as entity, then `.observe(save_to_disk(path))` - this is the 0.18 pattern
- **AppExit**: Use `MessageWriter<AppExit>` (not `EventWriter`) to exit the app
- **KeyboardInput events**: Use `MessageReader<KeyboardInput>` (not `EventReader`) in 0.18
- **Mesh3d + MeshMaterial3d**: Bevy 0.18 uses these component wrappers, not the old `PbrBundle`
- **Children iteration**: `children.iter()` returns `Entity` references directly
- **Gizmos axes**: `gizmos.axes(Transform::IDENTITY, length)` draws colored XYZ axes
- **Plane3d**: `Plane3d::new(Vec3::Y, Vec2::splat(size))` for ground planes
- **Text**: Use `Text::new(str)` component directly, not `TextBundle`

## Architecture Patterns

- **Multi-doodle architecture**: `AppState` enum with one variant per doodle; each doodle is a Bevy `Plugin` that self-registers `OnEnter`/`Update` systems gated on its state
- **Entity cleanup**: `DespawnOnExit(AppState::X)` on root entities; children auto-despawn with parent. Debug UI is persistent (no DespawnOnExit)
- **Module structure**: `shared/` (text_input, debug) for generic systems; `doodles/cubes/` (components, scene, ui, mod) for doodle-specific code; `menu.rs` for main menu; `app_state.rs` for state enum
- **Adding a new doodle**: New module in `doodles/`, new `AppState` variant, new plugin registered in `main.rs`, new menu button in `menu.rs`
- **Input -> Transform flow**: Text inputs store values as strings, systems read them, parse to f32, apply to transforms each frame
- **Bidirectional sync**: Main cube rotation syncs back to text inputs (except during auto-rotation or when input is focused, to avoid feedback loops)
- **UI toggle pattern**: `ToggleableUi` marker component on all hideable UI nodes, toggled via `Display::Flex`/`Display::None`
- **Constants at top of each module**: Extracted named constants for all magic numbers (colors, sizes, positions, speeds)
- **Navigate back**: `navigate_back` system runs `.before(handle_keyboard_input)` for correct two-press Escape behavior (first unfocuses input, second navigates)

## Gotchas

- **Quaternion -> Euler feedback loop**: When syncing rotation back to text inputs, must skip when auto-rotation is enabled or input is focused, otherwise values jitter
- **EulerRot::XYZ order matters**: Consistent use of XYZ order throughout for Euler conversions
- **Hue interpolation wrap-around**: Hue values are circular (0.0 == 1.0), need special lerp that takes the short path around the color wheel
- **Duplicate constants**: `CUBE_CONFIG` in `cubes/ui.rs` and `MAIN_CUBE_INITIAL_ROTATION` in `cubes/scene.rs` must stay in sync manually
- **DespawnOnExit**: Bevy 0.18 uses `DespawnOnExit<S>(pub S)` (not `StateScoped`). Children of despawned parents are auto-despawned
- **Camera per state**: Each state spawns its own camera with `DespawnOnExit`; old camera despawned on exit before new one spawns on enter
- **Stale InputFocusState**: When entities are despawned (state transition), `focused_entity` may reference a dead entity. Use `cleanup_stale_focus` system to auto-clear

## Nix/Devenv Notes

- **Devenv `languages.rust.channel`** requires `rust-overlay` input: `devenv inputs add rust-overlay github:oxalica/rust-overlay --follows nixpkgs`
- **Nixpkgs xorg deprecation**: `xorg.libX11` → `libx11`, `xorg.libXcursor` → `libxcursor`, `xorg.libXi` → `libxi`, `xorg.libXrandr` → `libxrandr` (as of nixpkgs-unstable Feb 2026)
- **Cachix binary cache**: Requires trusted-user status in NixOS config; disable with `cachix.enable = false` if not available

## User Preferences

- Prefers iterative development with visual verification via screenshots
- Likes subtle animations (slow 20s color transitions, not flashy)
- Values clean module separation
- Prefers concise documentation
