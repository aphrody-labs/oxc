# aphrody-labs/oxc

Fork of [oxc-project/oxc](https://github.com/oxc-project/oxc) used by aphrody-labs/bun. It is the source of truth for the
Oxc version the fork ships: `packages/bun-oxc` (crate `aphrody-oxc-bridge`, addon `@aphrody/bun-plugin-oxc`) and the
core `bun lint` / `bun fmt` commands.

- `main` follows upstream `main` (`.github/workflows/aphrody-upstream-sync.yml`, every 6 h) and mirrors the release
  tags `crates_v*`, `oxlint_v*`, `oxfmt_v*`. Upstream workflows are disabled here (`gh workflow disable`); only the
  sync runs. It pushes with the `APHRODY_SYNC_TOKEN` secret (scopes `repo`, `workflow`).
- `aphrody` starts on the release commit Bun pins and carries the fork's own patches, if any. Pinned release:
  crates `0.153.0`, oxlint `1.87.0`, oxfmt `0.72.0` = `2bd08ebe8f36fcf1954a675ffdeb4c6d0129f609` (tag `oxlint_v1.87.0`).

## How Bun consumes it

The published crates (`oxc_parser`, `oxc_transformer`, ...) come from crates.io at `=0.153.0`. `oxc_linter` and
`oxc_formatter` are not published, so `packages/bun-oxc/Cargo.toml` takes them from this repository by `rev` and
redirects every `oxc_*` crate to the same `rev` with `[patch.crates-io]`, so one copy of the AST is linked. A fork patch
goes on `aphrody`, and Bun moves its `rev` to the new `aphrody` head.

## Moving to a new Oxc release

1. Wait for the sync to mirror the new tags (or run the workflow by hand).
2. `git switch aphrody && git rebase --onto <new oxlint_v tag> <old tag>`, then `git push --force-with-lease origin aphrody`.
3. In aphrody-labs/bun: bump `oxc_*` in `packages/bun-oxc/Cargo.toml`, the `rev`, and the version line at the top of
   section L of `PLAN-ALPINE-BUN.md`.
