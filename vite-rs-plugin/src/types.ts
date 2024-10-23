import { ConfigEnv, Plugin, UserConfig } from 'vite'
import type { Config as FullReloadConfig } from 'vite-plugin-full-reload'

type DevServerUrl = `${'http' | 'https'}://${string}:${number}`

export interface ViteRustPlugin extends Plugin {
  config: (config: UserConfig, env: ConfigEnv) => UserConfig
}

export type ViteRustPluginInput = {
  /**
   * The Vite development server URL.
   *
   * @default "http://localhost:5173"
   */
  devServerUrl?: DevServerUrl,
  /**
   * Configures the files to watch during development for fast-refresh.
   * Can be a path to a directory or a file, an array of paths or a
   * `FullReloadConfig`. See
   * {@link https://github.com/ElMassimo/vite-plugin-full-reload|vite-plugin-full-reload}
   * for more details.
   *
   * @default entrypoints
   */
  refresh?: string | string[] | { paths: string[], config?: FullReloadConfig },

  /**
   * The entrypoints of your application. Will be set to Vite's
   * config `build.rollup.input` property.
   */
  entrypoints: string[] | string,
  /**
   * Directory where the bundle will be placed at.
   * Overrides Vite's config `build.outDir` property,
   * and acts like an alias to Vite's option.
   *
   * @default "dist"
   */
  outDir?: string,
  /**
   * The endpoint that serves static assets from the Vite's config public
   * directory.
   *
   * Useful when you cannot serve static assets at "/" for some reason, but
   * don't want to explicitly force an assets endpoint by setting
   * `assetsEndpoint`.
   *
   * @default "/"
   */
  publicEndpoint?: string,
  /**
   * The endpoint that serves static assets in `Manifest` (production) mode.
   *
   * If not set, a environment variable `ASSET_URL` will be used when vite is
   * run with `build` command and is on production mode. If the environment
   * variable does not exist, defaults to `false`.
   *
   * If set to `false`, the endpoint will be constructed by concatenating
   * `publicDir` and `outDir`.
   *
   * @default false.
   */
  assetsEndpoint?: string | false,
}

export type ResolvedPluginConfig = Required<Omit<
    ViteRustPluginInput, 'refresh' | 'entrypoints'
  >> & {
    refresh: { paths: string[], config?: FullReloadConfig },
    entrypoints: string[]
  }
