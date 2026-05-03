# CLAUDE.md

## Overview

Personal learning repo for experimenting with languages and tools. No production concerns — focus is exploration and working code.

## Key Directories

- `bevy/bevy_doodles/` — Active Rust/Bevy 0.18 project (game engine doodles)
- `rust/` — Rust exercises and experiments
- `c/`, `cpp/`, `haskell/`, `lua/` — Language explorations
- `python/`, `qwik-app/`, `rxjs/` — Scripting and frontend experiments
- `tauri/`, `ratatui/`, `electron/` — App framework explorations

## Key Commands

Most subdirs are self-contained. For the active Bevy project:

```bash
cd bevy/bevy_doodles
devenv shell        # Enter Nix dev environment (provides Rust + system libs)
cargo run           # Run the Bevy app
cargo build         # Build
cargo fmt           # Format
cargo clippy        # Lint
```

## Verification

After code changes in `bevy/bevy_doodles/`: `cargo clippy && cargo build`

## Voice

Terse by default.

1. **Ultra-short** — 1-4 words max per sentence. One-word answers preferred.
2. **No filler** — no preamble, no pleasantries, no narration.
3. **Tools first** — run tools, show result, stop.
4. **Drop pronouns and articles** — "Fixed." not "I fixed the code."
5. **Answers are telegrams** — strip every word that doesn't add information.

Say **"talk normal"** to switch to verbose. Say **"use terse voice"** to re-enable.

## Behavioral Rules

- **Think before coding** — state assumptions explicitly. If multiple approaches exist, present them. If unclear, ask.
- **Simplicity first** — NEVER add features beyond what was asked. No speculative abstractions.
- **Surgical changes** — touch only what you must. Match existing style.
- **Goal-driven execution** — frame tasks as verifiable success criteria, loop until verified.

## Gotchas

- Bevy project requires the `devenv` shell for `LD_LIBRARY_PATH` to resolve Vulkan/ALSA/Wayland libs — `cargo run` outside devenv will fail with linker errors.
- Each subdir is an independent project — there is no workspace-level Cargo.toml.
