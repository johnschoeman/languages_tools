# Coding Conventions

Design conventions that require judgment — things linters can't enforce.

## Universal

- ALWAYS prefer simplicity — implement only what's requested, no speculative features
- ALWAYS match existing patterns in the codebase before inventing new ones
- Use early returns to reduce nesting
- Small functions (<40 lines) and small files (<500 lines)
- Name things for the domain, not the implementation
- Prefer composition over inheritance

## Project-Specific

<!-- Add conventions specific to this project below. Examples: -->
<!-- - Bevy: ALWAYS use component-driven design, avoid putting logic in systems that should be in components -->
<!-- - Rust: NEVER use unwrap() in library code — use proper error propagation -->
