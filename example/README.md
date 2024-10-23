# ✨ Rust ✨ + React + TypeScript + Vite

This application was generated using Vite's cli. It then had the directory tree modified to fit with a cargo application (generated with cargo cli).

This example is a playground containing an application working with `vite-rust` integration.

## Running

You will need two processes for running a vite application with any other language but javascript. So, in one
powershell or shell tab, run:
```bash
# starts Vite dev server, which will serve the assets during development
$ npm run dev
```

In a second terminal, use `cargo run` to start your real application.

## Build

By running `npm run build` (or `vite build`), it will generate a manifest used by `vite-rust` when
on `Manifest` mode. After built, just type `cargo run` normally.

Note that, if you did not set `ViteMode` to `Manifest` and your vite dev-server isn't running at all,
an error log will be printted to your terminal and Vite will automatically swipe to `Manifest` mode,
in which, as described at the crate's `README.md` file, uses the build-generated manifest json file
to inject the modules in your html template.
