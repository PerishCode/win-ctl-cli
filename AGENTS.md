# AGENTS

## Core Principle

No Magic. Thin Core. Explicit Helpers.

- Prefer explicit behavior over implicit orchestration.
- Keep the Rust core thin; move ecosystem-specific work into helpers.
- Avoid generalized protocols when a concrete helper is enough.
- Reduce command surface area aggressively.
- Choose clear lifecycle verbs over abstract method layers.
- Keep `win-ctl-cli` as a repository skeleton unless a task explicitly asks for product logic.

## Directory Conventions

- `app/src/bin/win-ctl-cli.rs`: CLI entrypoint.
- `app/src/core/`: reserved for core runtime modules when the project grows.
- `app/src/commands/`: reserved for concrete subcommand implementations.
- `scripts/release/`: release acceptance, verification, packaging, checksum, and collation helpers.
- `scripts/docs/`: docs integrity and agent-entry validation helpers.
- `scripts/e2e/`: container-backed end-to-end smoke helpers.
- `scripts/manage/`: local install lifecycle helpers (`install`, `uninstall`).
- `helpers/`: compatibility-layer helper implementations; keep helper/runtime shims out of `scripts/`.
- `docker/`: container fixtures for helper validation and clean-room local testing.
- `docker-compose.yml`: long-lived workspace containers for local helper validation.
- Top-level `scripts/`: only user-facing entrypoints or cross-cutting helpers that do not fit a domain subdirectory.
- `docs/how-to/`: retained install/use task docs.
- `docs/explanation/`: scoreboard-facing docs only.
- `docs/posts/`: public design and iteration writeups when needed.
- `docs/changelog/`: release-only records that support publishing and self-update flows.
- `docs/zh-CN/`: must mirror the retained English docs surface, not exceed it.
- `app/examples/`: runnable sample configurations if the CLI later needs them.
- `docs/package.json`: docs workspace package manifest; keep docs toolchain isolated under the pnpm workspace.
- `target/`: local build outputs (generated, do not hand-edit).
- `.task/`: branch-bound task state for development workflow, must not stay on `main`.

### Placement Rules

- Prefer creating a domain subdirectory before adding a second script in the same area.
- New release helpers use single-word names when possible (`accept`, `verify`, `package`, `checksum`, `collate`).
- New docs helpers use short noun-style names when possible (`links`, `alignment`, `agent-meta`, `agent-routes`).
- Avoid adding new top-level docs categories unless they are part of the public minimal surface.
- Do not expand placeholder docs into broad reference/tutorial sprawl without an explicit product decision.

### Docker Helper Validation

- Use `docker compose up -d node-helper builder` to start the clean-room validation containers.
- Use `docker compose exec builder bash` when you need a modern Rust toolchain for `cargo build`.
- Use `docker compose exec node-helper bash` to enter the long-lived runtime validation container.
- Recommended split:
  - build in `builder`
  - validate docs/helper shape in `node-helper`
- The persistent `node-helper` image is defined in `docker/node-helper.Dockerfile`; prefer that over inline package-install bootstrap commands.
- Use `docker compose down` to stop and remove the containers.

## Helper Positioning

- `helper` is a tactical compatibility layer, not a long-term extension protocol.
- Do not design or document helpers as a third-party extension ecosystem.
- A helper exists to absorb ecosystem-specific operational mess until a tool becomes natively compatible.
- If an upstream tool becomes directly compatible, prefer deleting the corresponding helper instead of preserving compatibility abstractions.
- Keep helper contracts thin: `win-ctl-cli` is responsible for locating, caching, and executing helpers; helper-specific behavior lives in source + docs, not in a generalized helper spec.

## Development Workflow

1. Create or switch to a feature branch before changes.
2. Implement changes in `app/src/` and keep docs aligned when CLI behavior changes.
3. Run local checks before commit:
   - `cargo fmt --check`
   - `cargo test`
   - `pnpm run docs:build`
4. Keep `README.md` focused on user-facing usage and repository navigation.
5. Before merging to `main`, ensure `.task/` is cleaned up.

## Commit and Merge Rules

- Prefer small, focused commits with clear messages.
- Open PRs against `main`.
- Use squash merge to keep `main` history clean.

## Agent Autonomy Policy

- This repository is used for high-autonomy, closed-loop agent experiments.
- Required guardrails and process permissions are already in place for this purpose.
- Agents may create and manage PRs proactively when task flow requires it.
- Keep safety constraints active: no destructive git operations on shared history, and preserve reproducible verification steps.

## Node and pnpm Constraints

- Node.js version constraint: `^24`.
- pnpm version constraint: `^10`.
- Minor and patch differences are acceptable within the allowed major versions.
- For Node/docs/frontend workflows, prefer using `pnpm` consistently (`pnpm install`, `pnpm run docs:build`, `pnpm exec ...`).

## Cargo Environment Constraints

- Keep the Rust workspace minimal and compileable.
- `app/` is the only member unless a task explicitly expands it.
- Do not add runtime dependencies unless required.
- The binary name must stay `win-ctl-cli`.

## Docs Audience Policy

- Default assumption: foreseeable iterations are agent-driven; docs are optimized for agent consumption and execution efficiency.
- Human-oriented narration is optional and secondary; keep it only when it improves direct task closure.
- Prefer machine-actionable structure: clear entrypoints, deterministic labels, stable linking, minimal ambiguity.
