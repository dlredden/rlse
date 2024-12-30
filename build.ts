import type { BuildConfig } from "bun";
import dts from "bun-plugin-dts";

const defaults: BuildConfig = {
  entrypoints: ["./src/index.ts"],
  plugins: [dts()],
  naming: "[name].js",
  outdir: "./dist",
  minify: true,
  sourcemap: "linked",
};

await Promise.all([
  Bun.build({
    ...defaults,
    plugins: [dts()],
    target: "node",
  }),
  Bun.build({
    ...defaults,
    entrypoints: ["./src/browser.ts"],
    target: "browser",
  }),
]);
