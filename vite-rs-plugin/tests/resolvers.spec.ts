import {
  resolveBase,
  resolveViteRustPluginInputWithDefaults,
  trimPathSlashes,
} from "../src/helpers";
import type { ResolvedPluginConfig, ViteRustPluginInput } from "../src/types";

describe("Validate Plugin Config Resolver", () => {
  it("should fulfil optional fields with defaults", () => {
    const pluginConfig: ViteRustPluginInput = {
      entrypoints: "src/www/main.ts",
    };

    const resolvedWithDefaults =
        resolveViteRustPluginInputWithDefaults(pluginConfig);

    expect(resolvedWithDefaults).toMatchObject({
      assetsEndpoint: false,
      devServerUrl: "http://localhost:5173",
      entrypoints: ["src/www/main.ts"],
      outDir: "dist",
      publicEndpoint: "/",
      refresh: { paths: ["src/www/main.ts"] },
    } satisfies ResolvedPluginConfig);
  });

  it("should not override any of user options", () => {
    const pluginConfig: ViteRustPluginInput = {
      entrypoints: "src/www/main.ts",
      devServerUrl: "http://differrenthost.com:8080",
      assetsEndpoint: false,
      outDir: "output",
      publicEndpoint: "/public/",
      refresh: ["src/www/*"],
    };

    const resolvedWithDefaults =
          resolveViteRustPluginInputWithDefaults(pluginConfig);

    expect(resolvedWithDefaults).toMatchObject({
      entrypoints: ["src/www/main.ts"],
      devServerUrl: "http://differrenthost.com:8080",
      assetsEndpoint: false,
      outDir: "output",
      publicEndpoint: "/public/",
      refresh: { paths: ["src/www/*"] },
    } satisfies ResolvedPluginConfig);
  });
});

describe("Validate 'resolveBase' behaviour", () => {
  it("should never override user's vite config 'base' property", () => {
    const pluginConfig: ViteRustPluginInput = { entrypoints: "src/www/main.ts" };
    const resolvedConfigs = resolveViteRustPluginInputWithDefaults(pluginConfig);
    const resolved = resolveBase("/", resolvedConfigs, { command: "build", mode: "production" });
    expect(resolved).toBe("/");
  });

  it("should resolve to empty string when in dev-server, unless 'build' is set", () => {
    const pluginConfig: ViteRustPluginInput = {
      entrypoints: "src/www/main.ts",
      assetsEndpoint: "/assetsendpoint",
    };
    const resolvedConfigs = resolveViteRustPluginInputWithDefaults(pluginConfig);
    // provides no "build.base"
    const resolved = resolveBase(
      undefined,
      resolvedConfigs,
      { command: "serve", mode: "development" },
    );
    expect(resolved).toBe("");
  });

  it("should resolve to plugin's config 'assetsEndpoint' property", () => {
    const pluginConfig: ViteRustPluginInput = {
      entrypoints: "src/www/main.ts",
      assetsEndpoint: "/assetsendpoint",
    };
    const resolvedConfigs = resolveViteRustPluginInputWithDefaults(pluginConfig);
    // provides no "build.base"
    // build command
    const resolved = resolveBase(
      undefined,
      resolvedConfigs,
      { command: "build", mode: "production" },
    );
    expect(resolved).toBe("/assetsendpoint");
  });

  it("should resolve to concatenation of output and public directories", () => {
    const pluginConfig: ViteRustPluginInput = { entrypoints: "src/www/main.ts" };
    const resolvedConfigs = resolveViteRustPluginInputWithDefaults(pluginConfig);
    // provides no vite config "build.base"
    // build command
    // provides no plugin config "assetsEndpoint"
    const resolved = resolveBase(
      undefined,
      resolvedConfigs,
      { command: "build", mode: "production" },
    );
    expect(resolved.toString().replaceAll("\\", "/")).toBe("/dist");
  });
});

test("Validate 'trimPathSlashes' behaviour", () => {
  expect(trimPathSlashes("/dist/")).toBe("dist");
  expect(trimPathSlashes("/dist")).toBe("dist");
  expect(trimPathSlashes("dist/")).toBe("dist");
  expect(trimPathSlashes("\\dist\\")).toBe("dist");
  expect(trimPathSlashes("\\dist")).toBe("dist");
  expect(trimPathSlashes("dist\\")).toBe("dist");
  expect(trimPathSlashes("/")).toBe("");
  expect(trimPathSlashes("\\")).toBe("");
});
