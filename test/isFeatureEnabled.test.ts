import { expect, test } from "bun:test";
import { isFeatureEnabled } from "../src/index";

const config = {
  features: {
    testFeatureDevTest: { environments: ["dev", "test"] },
    testFeatureProd: { environments: ["prod"] },
  },
};

test("Feature with defaults", () => {
  expect(isFeatureEnabled({ feature: "testFeatureDevTest" })).toBe(true);
  expect(isFeatureEnabled({ feature: "testFeatureProd" })).toBe(false);
  expect(isFeatureEnabled({ feature: "missingFeature" })).toBe(false);
});

test("Feature with env", () => {
  expect(
    isFeatureEnabled({
      feature: "testFeatureDevTest",
      env: "dev",
    })
  ).toBe(true);
  expect(
    isFeatureEnabled({
      feature: "missingFeature",
      env: "dev",
    })
  ).toBe(false);
  expect(
    isFeatureEnabled({
      feature: "testFeatureDevTest",
      env: "test",
      config,
    })
  ).toBe(true);
  expect(
    isFeatureEnabled({
      feature: "testFeatureDevTest",
      env: "prod",
      config,
    })
  ).toBe(false);
  expect(
    isFeatureEnabled({
      feature: "missingFeature",
      env: "dev",
      config,
    })
  ).toBe(false);
});

test("Feature with config", () => {
  expect(
    isFeatureEnabled({
      feature: "testFeatureDevTest",
      config,
    })
  ).toBe(true);
  expect(
    isFeatureEnabled({
      feature: "testFeatureDevTest",
      env: "test",
      config,
    })
  ).toBe(true);
  expect(isFeatureEnabled({ feature: "missingFeature", config })).toBe(false);
  expect(
    isFeatureEnabled({
      feature: "missingFeature",
      env: "dev",
      config,
    })
  ).toBe(false);
});
