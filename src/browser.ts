import { getConfig } from "./config";
export { initConfig } from "./config";

/**
 * The properties for the `isFeatureEnabled` function.
 * @property {string} feature - The feature to check.
 * @property {string} [env] - The environment to check against.
 */
type IsFeatureEnabledProps = {
  feature: string;
  env?: string;
};

/**
 * @param {IsFeatureEnabledProps} options - The feature to check and the configuration to check against.
 * @returns {boolean} Whether the feature is enabled in the given environment.
 */
export async function isFeatureEnabled(
  options: IsFeatureEnabledProps
): Promise<boolean> {
  let cfg = await getConfig();
  let environment = options.env;

  if (!environment) {
    if (process.env.APP_ENV) {
      environment = process.env.APP_ENV;
    } else if (process.env.VERCEL_ENV) {
      environment = process.env.VERCEL_ENV;
    } else {
      environment = "dev";
    }
  }

  const feature_config = cfg.features[options.feature];
  if (!feature_config) return false;
  return feature_config.environments.includes(environment);
}
