import { expect, test } from "bun:test";
import { isFeatureEnabled } from "../src/index";

const config = {
  features: {
    testFeatureDevTest: { environments: ["dev", "test"] },
    testFeatureProd: { environments: ["prod"] },
  },
};

test("Feature with defaults", async () => {
  expect(await isFeatureEnabled({ feature: "testFeatureDevTest" })).toBe(true);
  expect(await isFeatureEnabled({ feature: "testFeatureProd" })).toBe(false);
  expect(await isFeatureEnabled({ feature: "missingFeature" })).toBe(false);
});

test("Feature with env", async () => {
  expect(
    await isFeatureEnabled({
      feature: "testFeatureDevTest",
      env: "dev",
    })
  ).toBe(true);
  expect(
    await isFeatureEnabled({
      feature: "missingFeature",
      env: "dev",
    })
  ).toBe(false);
  expect(
    await isFeatureEnabled({
      feature: "testFeatureDevTest",
      env: "test",
      config,
    })
  ).toBe(true);
  expect(
    await isFeatureEnabled({
      feature: "testFeatureDevTest",
      env: "prod",
      config,
    })
  ).toBe(false);
  expect(
    await isFeatureEnabled({
      feature: "missingFeature",
      env: "dev",
      config,
    })
  ).toBe(false);
});

test("Feature with config", async () => {
  expect(
    await isFeatureEnabled({
      feature: "testFeatureDevTest",
      config,
    })
  ).toBe(true);
  expect(
    await isFeatureEnabled({
      feature: "testFeatureDevTest",
      env: "test",
      config,
    })
  ).toBe(true);
  expect(await isFeatureEnabled({ feature: "missingFeature", config })).toBe(
    false
  );
  expect(
    await isFeatureEnabled({
      feature: "missingFeature",
      env: "dev",
      config,
    })
  ).toBe(false);
});
