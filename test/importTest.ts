import path from "path";
import { promises as fs } from "fs";
import { isFeatureEnabled, parseConfig } from "../dist";

async function test() {
  const config = parseConfig(
    await fs.readFile(path.resolve(process.cwd(), "rlse.toml"), "utf-8")
  );
  if (await isFeatureEnabled({ feature: "testFeatureDevTest", config })) {
    console.log("testFeatureDevTest enabled!");
  }

  if (
    await isFeatureEnabled({
      feature: "testFeatureDevTest",
      config,
      env: "prod",
    })
  ) {
    console.log("testFeatureDevTest enabled?");
  } else {
    console.log("testFeatureDevTest disabled!");
  }
}

test();
