# Changelog
Important changes will be mentioned in this file. The project adopts
[SemVer (Semantic Versioning 2.0.0)](https://semver.org/), as suggested
by Cargo for its crates.

## v0.2.3
- Add optional `app_url` field to customize the URL to fetch assets from;
- Looks for a `APP_URL` environment variable to use as `app_url` if none is explicitly provided on Vite Rust setup;
- Make assets path absolute, fixing an issue that could happen when accessing a route with 2 or more segments (browser would look for the asset in the first segment instead of the root).

## v0.2.2
- `ViteMode::discover()` now also looks for `RUST_ENV`, `LOCO_ENV` and `RAILS_ENV` environment variables;
- add `set_prefix` method to force a prefix to assets paths.

## v0.2.1
- Fix `ViteMode::discover()` heuristic when using `RUST_ENV`, `APP_ENV` and `NODE_ENV`;
- Manifest initializes even with `Development` mode if `entrypoints` aren't set. 

## v0.2.0
- Introduce fluent syntax to the `ViteConfig` struct initialization;
- Replaces `ViteConfig::new_with_defaults` by `ViteConfig::default` (`Default` trait implementation);
- "manifest.json" file will be only required if mode is "Manifest";
    - as Manifest is now optional, methods that requires it had their return type changed to `Result<T, ViteError>`;
- HeartBeat checker will retry *n* times before falling back Vite fallbacks to `Manifest` mode.

## v0.1.3
- Fix regex errors in `vite` and `react` basic directives. Note that v0.1.1 and v.0.1.2 don't completely fix them.
