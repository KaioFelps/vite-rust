import type { UserConfig } from "vite";

import rustVitePlugin from "../src";

describe("Validate Rust Vite Plugin behaviour", () => {
  it("should never override any of vite options but 'build.outDir'", () => {
    const userViteConfig: UserConfig = {
      base: "/my/base",
      build: {
        outDir: "should/be/overrided",
      },
    };

    const plugin = rustVitePlugin({
      entrypoints: "src/main.ts",
      outDir: "overrided/out/dir",
      assetsEndpoint: "/forced/endpoint/",
    })[0];

    const buildConfig = plugin.config(userViteConfig, { command: "build", mode: "production" });
    const devConfig = plugin.config(userViteConfig, { command: "serve", mode: "development" });

    expect(buildConfig.base).toBe("/my/base");
    expect(devConfig.base).toBe("/my/base");
    expect(buildConfig.build?.outDir).toBe("overrided/out/dir");
    expect(devConfig.build?.outDir).toBe("overrided/out/dir");
    expect(buildConfig.build?.manifest).toBe(true);
  });

  it("should not override user's manifest path, but set it if it isn't provided", () => {
    const userViteConfig: UserConfig = {
      base: "/my/base",
      build: {
        outDir: "should/be/overrided",
        manifest: false,
      },
    };

    const plugin = rustVitePlugin("src/main.ts")[0];
    const config = plugin.config(userViteConfig, { command: "build", mode: "production" });

    expect(config.build?.manifest).toBe(false);
  });
});
