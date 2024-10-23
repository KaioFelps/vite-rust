import path from 'node:path'
import type {
  ConfigEnv,
  ResolvedConfig,
  UserConfig,
} from 'vite'
import {
  ResolvedPluginConfig,
  ViteRustPlugin,
  ViteRustPluginInput,
} from './types'

export function resolveRustVitePlugin(
  pluginConfig: ResolvedPluginConfig,
): ViteRustPlugin {
  const originPlaceholder = '__rust_vite_plugin_placeholder__'
  let resolvedConfig: ResolvedConfig

  return {
    name: 'vite-rust-plugin',
    enforce: 'post',

    config: (config, env) => {
      return {
        base: resolveBase(config.base, pluginConfig, env),
        build: {
          outDir: pluginConfig.outDir, // where the build will be placed at
          rollupOptions: {
            input: pluginConfig.entrypoints,
          },
          manifest: config.build?.manifest === undefined
            ? true
            : config.build.manifest,
        },
        server: {
          origin: config.server?.origin ?? originPlaceholder,
        },
      } as Omit<UserConfig, 'plugins'>
    },

    configResolved(config) {
      resolvedConfig = config
    },

    transform(code) {
      if (resolvedConfig.command === 'serve') {
        const newCode = code.replace(
          originPlaceholder,
          pluginConfig.devServerUrl,
        )

        return {
          code: newCode,
        }
      }

      return null
    },
  }
}

export function resolveViteRustPluginInputWithDefaults(
  config: ViteRustPluginInput,
): ResolvedPluginConfig {
  const entrypoints = Array.isArray(config.entrypoints)
    ? config.entrypoints
    : [config.entrypoints]

  return {
    devServerUrl: config.devServerUrl ?? 'http://localhost:5173',
    entrypoints,
    refresh: resolveRefreshConfig(config, entrypoints),
    assetsEndpoint: config.assetsEndpoint ?? false,
    publicEndpoint: config.publicEndpoint ?? '/',
    outDir: config.outDir
      ? trimPathSlashes(config.outDir)
      : 'dist',
  }
}

export function resolveBase(
  userBase: string | undefined,
  config: ResolvedPluginConfig,
  env: ConfigEnv,
) {
  const configAssetsEndpoint = resolveAssetsEndpoint(config, env)
  if (userBase) return userBase
  if (env.command === 'serve') return ''
  if (configAssetsEndpoint) return config.assetsEndpoint

  return path.join(config.publicEndpoint, config.outDir)
}

function resolveAssetsEndpoint(
  config: ViteRustPluginInput,
  env: ConfigEnv,
): string | false {
  if (config.assetsEndpoint === false) return false
  if (config.assetsEndpoint) return trimPathSlashes(config.assetsEndpoint) + '/'

  if (
    process.env.ASSET_URL &&
    env.command === 'build' &&
    env.mode === 'production'
  ) return trimPathSlashes(process.env.ASSET_URL) + '/'

  return false
}

export function trimPathSlashes(input: string) {
  input = input.replaceAll('\\', '/')

  if (input === '/' || input === '//') return ''

  if (input.startsWith('/')) input = input.slice(1)
  if (input.endsWith('/')) input = input.slice(0, input.length - 1)

  return input
}

export function resolveRefreshConfig(
  config: ViteRustPluginInput,
  entrypoints: string[],
) {
  switch (typeof config.refresh) {
    case 'undefined':
      return { paths: entrypoints }
    case 'string':
      return { paths: [config.refresh] }
    case 'object':
      return Array.isArray(config.refresh)
        ? { paths: config.refresh }
        : config.refresh
  }
}
