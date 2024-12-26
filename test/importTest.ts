import path from "path";
import fs from "fs/promises";
import { isFeatureEnabled, initConfig } from "../dist/browser";

async function test() {
  initConfig(
    await fs.readFile(path.resolve(process.cwd(), "rlse.toml"), "utf-8")
  );
  if (await isFeatureEnabled({ feature: "testFeatureDevTest" })) {
    console.log("testFeatureDevTest enabled!");
  }

  if (
    await isFeatureEnabled({
      feature: "testFeatureDevTest",
      env: "prod",
    })
  ) {
    console.log("testFeatureDevTest enabled?");
  } else {
    console.log("testFeatureDevTest disabled!");
  }
}

test();
