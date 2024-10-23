# Vite Rust

A [Vite integration](https://vite.dev/guide/backend-integration) for
Rust back-ends.

This crate provides methods for parsing a Vite-generated `manifest.json` file into
meaningful HTML tags, as well as utility methods for injecting these tags and other Vite
scripts into HTML templates.

## Getting started
To get started, add the following under your Cargo `dependencies` field: 
```toml
vite-rust = { version = "0.1.x" } 
```

rust-vite provides a `Vite` struct that is responsible for dealing with the manifest and
its chunks, and also is your interface for generating the HTML you need.

To initialize it, you need to provide a `ViteConfig` struct that contains data that will
be used to manage the manifest and also generate the HTML tags according to the **mode** you
are running at.

```rust
let mut vite_config: vite_rust::ViteConfig = vite_rust::ViteConfig::new_with_defaults("path/to/manifest.json");
vite_config.entrypoints = Some(vec!["src/main.tsx", "src/index.css"]);

let vite = vite_rust::Vite::new(vite_config).await.unwrap();

let tags = vite.get_tags(); // get html tags from Manifest and given entrypoints
let hmr_script = vite.get_hmr_script(); // get hmr script if Mode is set to Develpment. Empty string otherwise

// inject this manually to your html somehow...
```

## Vite config
We expect you to have `vite.config.ts` file created and set up by yourself. Unlike some
other plugins, such as Innocenzi's Laravel plugin, we won't generate the config file
from our `ViteConfig` struct for you.

Nevertheless, here are two recommendations for getting it set up more easily:

1. [laravel/vite-plugin](https://github.com/laravel/vite-plugin)    - A robust and very popular
Vite plugin for setting up Vite with Laravel applications. With a few extra settings,
you can get it working with your Rust application!

2. [vite-rs-plugin](./vite-rs-plugin/)  - A pretty simple plugin that is enough for getting
your Rust application running. 

## Integrations and directives
At this point, vite-rust only provides a really basic HTML directives set. You can
use them by enabling the `basic_directives` feature at features property inside your
Cargo file:

```toml
[dependencies]
vite-rust = { version = "0.1.x", features = ["basic_directives"] } 
```

However, `Vite` struct also provide many helper methods that might be useful if you want to
**create your own template-engine specific helpers, directives or integration**.

Our basic directives work by receiving an HTML string reference and replaces the
plain text directives using Regex:

```rust
use vite_rust::{ Vite, ViteConfig, features::html_directives::ViteDefaultDirectives };

let vite = Vite::new(ViteConfig::new_with_defaults("path/to/manifest.json"));

let my_html = r#"
<htmL>
    <head>
        @vite::react
        @vite
    </head>
    <body>
        <div id="root"></div>
    </body>
</html>
"#;

state.vite.vite_directive(&mut template);
state.vite.react_directive(&mut template);
```

## Little helper for manifest path
We provide a little path resolver function for finding the manifest file.
It is experimental and bugs might be found, though:

```rust
use vite_rust::{utils::resolve_path, ViteConfig};

// if path is invalid, or file do not exist, the function will panic
let manifest_path: String = resolve_path(file!(), "../dist/.vite/manifest.json");
let vite_config: ViteConfig = ViteConfig::new_with_defaults(&manifest_path);
```

## What on earth is a mode?
There is an enum called ViteMode, that can be either `Development` or `Manifest`.
It indicates to vite-rust how to manage your assets.

If in `Development`, a few things happen:
- your assets will be resolved to reference the assets served by Vite development server.
Hence, it expects the server to be running or some unexpected behaviour might occur.
- some of Vite methods might return different strings. `get_resolved_vite_scripts`, for instance,
returns the HMR script and a module script referencing your entrypoint.

If in `Manifest` mode, other things happen:
- by using our basic directive, "@vite" will be replaced by HTML tags referencing bundled scripts
needed to make your application run, as some preload links for enabling faster loading;
- some of Vite methods will conditionally return different output according to this option. `get_resolved_vite_scripts`
will return, instead of HMR and client scripts, the HTML assets tags mentioned above.

It means that if you intend to make your own directives, this is the key for correctly
expanding them.

The mode can be set at `ViteConfig`'s `force_mode` property. It defaults to `None`. On Vite initialization,
if there is not a required mode, it will discover what mode to run based on a few steps that includes:

1. Checking for "production" value in environment variables such as "NODE_ENV", "APP_ENV" or "RUST_ENV";
2. Checking if heart beat checking is enabled at the config struct (if not, Development mode will be used);
3. Checking if development server is enabled (if not, Manifest mode will be used);
4. Pinging Vite's development server to check whether it is running.

Every method is documented and explains what you can expect from it.

## Examples
So far, there is only one really simple sample application available at [/example](./example/) directory.

## Contributing, docs, issues...
If you find any bug with the application, please create an Issue or feel free to make a pull request.
I will be working on providing templates for contributing as soon as possible.

More docs about every method available can be found at docs.rs.

Also I want to credit some amazing projects that served as references:

- [Innocenzi/laravel-vite](https://github.com/innocenzi);
- [Laravel/vite-plugin](https://github.com/laravel/vite-plugin);
- [ElMassimo/vite_ruby](https://github.com/ElMassimo/vite_ruby);
- [HilmJulien/in-vite](https://github.com/HiImJulien/in-vite);
---

> [!WARNING]
> Unexpected bugs might occur, and vite-rust is barely newborn. Please
> be careful and let us know about any bug found.

> [!NOTE]
> While not reached version 1.x, commits will be pushed directly to the
> main branch.

## License
This project is licensed under the [MIT](./LICENSE) license.