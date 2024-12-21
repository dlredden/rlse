import type { BuildConfig } from "bun";
import dts from "bun-plugin-dts";

const defaultBuildConfig: BuildConfig = {
  entrypoints: ["./src/index.ts"],
  outdir: "./dist",
};

await Promise.all([
  // Bun.build({
  //   ...defaultBuildConfig,
  //   // entrypoints: ["./src/browser.ts"],
  //   outdir: "./dist",
  //   plugins: [dts()],
  //   target: "browser",
  //   format: "esm",
  //   naming: "[dir]/browser.[ext]",
  // }),
  Bun.build({
    ...defaultBuildConfig,
    // entrypoints: ["./src/index.ts"],
    outdir: "./dist",
    plugins: [dts()],
    // target: "node",
    // format: "esm",
  }),
]);
