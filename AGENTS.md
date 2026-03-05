# Diasko - Agent Guidelines

This document provides guidelines for AI agents working on the Diasko codebase. Diasko is a Tauri application with a TypeScript frontend and Rust backend.

## Build Commands

### Frontend (TypeScript)
- `bun run dev` - Start development server with Bun (serves `index.html`)
- `bun run build` - Build frontend with Bun
- `bun run tauri` - Run Tauri CLI (requires additional subcommand)

### Backend (Rust)
- `cargo build` - Build Rust code in debug mode
- `cargo build --release` - Build for release
- `cargo tauri dev` - Start Tauri development (requires `@tauri-apps/cli` installed globally or via `bun run tauri dev`)

### Combined
- `bun run tauri dev` - Start full Tauri development (frontend + backend)

## Lint & Format Commands

### TypeScript/JavaScript (Biome)
The project uses [Biome](https://biomejs.dev/) for linting and formatting. Configuration is in `biome.json`.

- `bunx biome check .` - Check code for lint and formatting issues (dry run)
- `bunx biome lint .` - Run linter only
- `bunx biome format .` - Format code
- `bunx biome check --apply .` - Apply safe fixes and format
- `bunx biome check --apply-unsafe .` - Apply unsafe fixes (use with caution)

Biome is configured with:
- Indent style: tabs
- Quote style: double quotes for JavaScript/TypeScript
- VCS integration enabled (respects `.gitignore`)

### Rust
- `cargo fmt` - Format Rust code with rustfmt (default settings)
- `cargo clippy` - Run Clippy lints (no custom configuration)

## Test Commands

### Rust
- `cargo test` - Run all Rust unit and integration tests
- `cargo test -- --nocapture` - Run tests with output printed
- `cargo test <test_name>` - Run a specific test

### Frontend
No test framework is currently set up. Consider adding Vitest or Jest in the future.

## Code Style Guidelines

### TypeScript/JavaScript

#### Imports
- Use ES modules (`import`/`export`)
- Group imports in the following order:
  1. External dependencies (e.g., `@tauri-apps/api`)
  2. Internal aliased paths (e.g., `@/types`, `@/features/*`)
  3. Relative imports (e.g., `../class/create.ts`)
- Use `import type` for type-only imports
- Example:
  ```ts
  import { invoke } from "@tauri-apps/api/core";
  import type { Task } from "@/types.ts";
  import { createButtonListTask } from "../class/create.ts";
  import { element } from "../class/element.ts";
  ```

#### Naming Conventions
- Variables and functions: `camelCase`
- Types and interfaces: `PascalCase`
- Constants: `camelCase` (or `UPPER_SNAKE_CASE` for true constants)
- Files: `kebab-case` for feature directories, `camelCase` for individual files (observed pattern: `event.ts`, `switchView.ts`, `valueGlobal.ts`)

#### Error Handling
- Use `throw new Error("descriptive message")` for synchronous errors
- Use `Promise.reject` or `throw` in async functions
- Prefer explicit error messages that help debugging

#### Formatting
- Follow Biome configuration (tabs, double quotes)
- Braces on same line (K&R style)
- Semicolons: yes (Biome default)
- Line length: not configured; keep lines reasonable (~80-100 chars)

#### Types
- Use strict TypeScript (`strict: true` in tsconfig)
- Avoid `any`; use specific types
- Define interfaces for object shapes
- Use `type` for unions, intersections, and aliases
- Example:
  ```ts
  export interface Task {
      id: number;
      name: string;
      text: string;
  }
  ```

### Rust

#### Imports
- Use `use` statements grouped with blank lines between external, internal, and super imports
- Prefer absolute paths within the crate (`crate::db::...`)
- Import traits explicitly when needed

#### Naming Conventions
- Follow Rust standard naming (snake_case for functions/variables, PascalCase for types, SCREAMING_SNAKE_CASE for constants)
- Use descriptive names; avoid abbreviations unless widely understood

#### Error Handling
- Use `Result<T, E>` for fallible operations
- Use `String` as error type for simplicity in this codebase (as observed)
- Propagate errors with `?` operator
- Provide context with `.map_err(|e| e.to_string())` when converting from library errors

#### Formatting
- Use `cargo fmt` to enforce standard Rust style
- Indent with 4 spaces (rustfmt default)
- Line width 100 characters (default)

#### Documentation
- Add doc comments (`///`) for public items
- Use `//` for inline comments where necessary

## Project Structure

- `src/` – Frontend TypeScript source
  - `features/` – Feature‑specific modules
  - `class/` – Utility classes and shared logic
- `src‑tauri/` – Rust backend
  - `src/` – Rust source (main.rs, lib.rs, db module)
  - `Cargo.toml` – Rust dependencies
- `index.html` – Entry point for the frontend
- `tsconfig.json` – TypeScript configuration with path aliases
- `biome.json` – Linting/formatting configuration

## Path Aliases (TypeScript)

Configured in `tsconfig.json`:
- `@/*` → `src/*`
- `@class/*` → `src/class/*`
- `@features/*` → `src/features/*`

Use these aliases for imports to avoid deep relative paths.

## Agent Workflow

1. **Before making changes** run `bunx biome check .` to ensure code style compliance.
2. **After making changes** run `bunx biome check --apply .` to auto‑fix lint and formatting issues.
3. **For Rust changes** run `cargo fmt` and `cargo clippy`.
4. **Run tests** with `cargo test` (frontend tests not yet implemented).
5. **Verify build** with `bun run build` and `cargo build`.

## Git Workflow for Agents

When working on the Diasko codebase, AI agents should follow this Git workflow to ensure clean branch management and easy integration:

### Starting a Session
1. **Create a new branch** when beginning a new task or session:
   - Branch naming convention: `agent/<description>-<timestamp>` (e.g., `agent/fix-editor-bug-20250305`)
   - Use descriptive names that summarize the work being done
   - Create the branch from the current `main` or default branch
   - Example command: `git checkout -b agent/fix-editor-bug-20250305`

2. **Verify branch creation** with `git status` to confirm you're on the new branch.

### During Development
- Make commits as you complete logical units of work
- Write clear, descriptive commit messages that explain the "why" not just the "what"
- Push your branch to the remote repository if needed for backup or collaboration

### Completing a Session
1. **Check mergeability** before finishing:
   - Fetch the latest changes from the base branch: `git fetch origin`
    - Check for conflicts: `git merge-base --is-ancestor HEAD origin/$(git symbolic-ref refs/remotes/origin/HEAD | sed 's@^refs/remotes/origin/@@') && echo "No conflicts" || echo "Potential conflicts"`
    - Alternatively, attempt a dry-run merge: `git merge --no-ff --no-commit origin/$(git symbolic-ref refs/remotes/origin/HEAD | sed 's@^refs/remotes/origin/@@')`
   - If conflicts are detected, inform the user and provide guidance on resolving them

2. **Inform the user** about the branch status:
   - Clearly state whether the branch can be merged cleanly
   - Provide the branch name and any necessary commands for merging
   - Offer to create a pull request if appropriate

3. **Provide next steps**:
    - Suggest merging the branch: `git checkout $(git symbolic-ref refs/remotes/origin/HEAD | sed 's@^refs/remotes/origin/@@') && git merge --no-ff agent/branch-name`
   - Recommend deleting the branch after merging: `git branch -d agent/branch-name`
   - If the branch cannot be merged cleanly, suggest conflict resolution steps

### Example Completion Message
```
I've completed the work on branch `agent/fix-editor-bug-20250305`. 

✅ **Merge Status**: The branch can be merged cleanly into the base branch (`$(git symbolic-ref refs/remotes/origin/HEAD | sed 's@^refs/remotes/origin/@@')`) with no conflicts.

**To merge this work**:
1. Switch to the base branch: `git checkout $(git symbolic-ref refs/remotes/origin/HEAD | sed 's@^refs/remotes/origin/@@')`
2. Merge the changes: `git merge --no-ff agent/fix-editor-bug-20250305`
3. Delete the feature branch: `git branch -d agent/fix-editor-bug-20250305`

Would you like me to proceed with creating a pull request or merging the branch?
```

## Notes

- No Cursor rules (`.cursor/rules/`) or Copilot instructions (`.github/copilot-instructions.md`) are present.
- The project uses Bun as the package manager/runtime (lockfile: `bun.lock`).
- Tauri v2 is used; refer to [Tauri documentation](https://tauri.app/) for platform‑specific considerations.
- **Dependency Policy**: Do not add new packages to `package.json` (JavaScript/TypeScript) or `Cargo.toml` (Rust) without explicit user approval. When functionality requires a new dependency, ask the user first.

## Adding New Features

- Follow existing patterns in the `features/` directory.
- For new Rust commands, add the function in the appropriate `db` module and register it in `lib.rs` `invoke_handler`.
- Ensure error messages are user‑friendly and logged appropriately.
- Update this guide if new conventions are established.

---

*Last updated: 2026-03-05*
