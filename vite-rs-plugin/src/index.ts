import type { Plugin } from 'vite'
import fullReload from 'vite-plugin-full-reload'
import { ViteRustPlugin, ViteRustPluginInput } from './types'
import {
  resolveRustVitePlugin,
  resolveViteRustPluginInputWithDefaults,
} from './helpers'

export default function rustVitePlugin(
  config : ViteRustPluginInput | string,
): [ViteRustPlugin, ...Plugin[]] {
  const pluginRawConfig: ViteRustPluginInput = typeof config === 'string'
    ? { entrypoints: config as string }
    : config as ViteRustPluginInput

  const pluginConfig = resolveViteRustPluginInputWithDefaults(pluginRawConfig)

  return [
    resolveRustVitePlugin(pluginConfig),
    fullReload(
      pluginConfig.refresh.paths,
      pluginConfig.refresh.config,
    ) as Plugin,
  ]
}
