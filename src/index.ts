import { getConfig } from "./config";
import { type Config } from "./types";

/**
 * The properties for the `isFeatureEnabled` function.
 * @property {string} feature - The feature to check.
 * @property {string} [env] - The environment to check against.
 * @property {Config} [config] - The configuration to check against.
 */
type IsFeatureEnabledProps = {
  feature: string;
  env?: string;
  config?: Config;
};

/**
 * @param {IsFeatureEnabledProps} options - The feature to check and the configuration to check against.
 * @returns {boolean} Whether the feature is enabled in the given environment.
 */
export function isFeatureEnabled(options: IsFeatureEnabledProps): boolean {
  let cfg = options.config ? options.config : getConfig();
  let environment = options.env
    ? options.env
    : process.env.APP_ENV
    ? process.env.APP_ENV
    : "dev";

  const feature_config = cfg.features[options.feature];
  if (!feature_config) return false;
  return feature_config.environments.includes(environment);
}
