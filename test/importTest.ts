import data from "../rlse.toml";
import { isFeatureEnabled } from "../dist";

if (isFeatureEnabled({ feature: "testFeatureDevTest", config: data })) {
  console.log("testFeatureDevTest enabled!");
}

if (
  isFeatureEnabled({ feature: "testFeatureDevTest", config: data, env: "prod" })
) {
  console.log("testFeatureDevTest enabled?");
} else {
  console.log("testFeatureDevTest disabled!");
}
