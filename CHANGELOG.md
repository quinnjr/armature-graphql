# Changelog — `armature-graphql`

All notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this crate adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Earlier changes are recorded in the workspace [`CHANGELOG.md`](../CHANGELOG.md).

## [Unreleased]

## [0.8.0] - 2026-10-29

### Changed

- Requires `armature-core` 0.12 (was `0.11`).

## [0.7.0] - 2026-10-17

### Security

- **Breaking:** `GraphQLConfig::new()` (and therefore `GraphQLConfig::default()`)
  now returns production-safe defaults: playground, GraphiQL, schema docs and
  introspection are off; query depth is capped at 15, complexity at 1000,
  directives per field at 10 and parser recursion at 32. `production()` is now
  identical to `new()`. Migration: use `GraphQLConfig::development()` or the
  `with_*` builders to opt in to developer tooling.
- **Breaking:** `SchemaBuilder`, `ProgrammaticSchemaBuilder` and
  `SubgraphSchemaBuilder` apply `GraphQLConfig::default()` when no config is
  attached, instead of building an unrestricted, introspectable schema.
  Migration: attach `.config(GraphQLConfig::development(..))` (or a config with
  `with_introspection(true)` / `with_max_depth(0)`) to restore the old behavior.
- The complexity limit also bounds alias amplification (every aliased field
  costs at least 1); async-graphql has no separate alias-count limit.
- GraphQL Playground (`graphql-playground-react` 1.7.28) and GraphiQL
  (`graphiql` 3.9.0, React 18.3.1) assets are pinned to exact versions and
  loaded with Subresource Integrity + `crossorigin="anonymous"`, replacing the
  unversioned CDN URLs. GraphiQL now mounts with `ReactDOM.createRoot`.
- The endpoint interpolated into the playground/GraphiQL pages is emitted as
  an escaped JavaScript string literal.

### Added

- `GraphQLConfig::max_directives` / `with_max_directives` and
  `max_recursive_depth` / `with_max_recursive_depth`, plus the
  `DEFAULT_MAX_*` constants and `GRAPHQL_PLAYGROUND_VERSION`,
  `GRAPHIQL_VERSION`, `GRAPHIQL_REACT_VERSION`. **Breaking:** new public fields
  on `GraphQLConfig` (struct-literal construction must add them).

## [0.6.0] - 2026-09-15

### Changed

- **Breaking:** requires `armature-core` 0.10 (was `0.9`); its types appear in this crate's API, so the requirement change is breaking here and the minor moves. Part of the `armature-core` 0.10 release train.
- Dependency requirements moved to their latest releases: `syn` 2 → 3 (optional,
  static SDL export), `tokio-tungstenite` 0.29 → 0.30 (optional), `tokio` 1.53.
  No source changes were required; static SDL output is unchanged.

## [0.5.0] - 2026-08-05

### Changed

- **Requires `armature-core` 0.9 (breaking).** The requirement moved `0.8` →
  `0.9`. `armature-core 0.9.0` itself moves `armature-h1` across a breaking
  0.x boundary; because `armature-core` types appear in this crate's own
  public API, the requirement change is breaking here too and the minor moves
  with it. Under Cargo's 0.x caret rules the 0.8 and 0.9 types are distinct
  and do not unify, so a consumer holding an `armature-core 0.8` type cannot
  pass it to this crate. Part of the `armature-core 0.9.0` release train; see
  `armature-core`'s CHANGELOG for the publish order.

## [0.4.2] - 2026-08-04

### Fixed

- Requirements on sibling armature crates name a minor instead of `0`. Under
  Cargo's 0.x rules `version = "0"` matches any release ever made, and edition
  2024 selects the MSRV-aware resolver, so a consumer declaring an older
  `rust-version` was handed the oldest version satisfying it — resolving
  `armature-core = "0"` on Rust 1.89 produced `armature-core 0.2.3` while an
  explicit `armature-core = "0.8"` elsewhere in the same graph pulled 0.8.2.
  Two copies of core, and a build failing on symbols the older one lacks. Each
  0.x minor in this family is a breaking change, so the requirement now names
  one. No API change.
