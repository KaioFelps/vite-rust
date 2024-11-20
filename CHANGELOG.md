# Changelog
Important changes will be mentioned in this file. The project adopts
[SemVer (Semantic Versioning 2.0.0)](https://semver.org/), as suggested
by Cargo for its crates.

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
