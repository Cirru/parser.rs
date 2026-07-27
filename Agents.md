# Agents.md

## Release checklist (cargo publish via GitHub Release)

`publish.yaml` runs `katyo/publish-crates@v1`, which calls `cargo publish`
**without** `--allow-dirty`. `cargo publish` refuses to run if any tracked
file (including `Cargo.lock`) differs from what's committed. Do NOT work
around this in CI (e.g. `git checkout Cargo.lock` / `--allow-dirty`) —
keeping `Cargo.lock` honest and in sync with `Cargo.toml` is a local
responsibility, not something CI should paper over.

Before bumping the version / tagging a release:

1. Bump `version` in `Cargo.toml`.
2. Run `cargo update -p cirru_parser` (or just `cargo build`/`cargo test`) so
   `Cargo.lock` picks up the new version — then `git add Cargo.toml Cargo.lock`
   together in the same commit.
3. Commit as `release X.Y.Z`, tag `X.Y.Z` (no `v` prefix), push branch + tag.
4. `gh release create X.Y.Z --title "X.Y.Z" --notes "..."` — this triggers
   `publish.yaml`.
5. Poll with `gh run list --limit 5` until the publish workflow succeeds;
   confirm with `cargo search cirru_parser --limit 1 --registry crates-io`.

If a publish run fails with "files in the working directory contain changes
that were not yet committed into git: Cargo.lock", it means the release
commit itself has a stale `Cargo.lock` — fix locally (step 2) and re-tag/
re-release; don't touch the workflow.
