import fs from "fs/promises";
import path from "path";
import toml from "toml";

import { type Config } from "./types";

let cachedConfig: Config | null = null;

/**
 * Initializes the configuration by parsing a TOML string.
 * @param {string} config The TOML configuration string.
 */
export function initConfig(config: string): void {
  cachedConfig = parseConfig(config);
}

/**
 * Retrieves the configuration, parsing it from a TOML file if not already cached.
 * @returns {Config} The parsed configuration object.
 */
export async function getConfig(): Promise<Config> {
  if (cachedConfig) {
    return cachedConfig; // Return cached configuration if already loaded
  }

  if (typeof window !== "undefined")
    throw new Error(
      "When using rlse in the browser, use `initConfig(config: string)` to pre-hydrate the config."
    );

  const configFile = process.env.RLSE_CONFIG || "rlse.toml";
  const fullPath = path.resolve(process.cwd(), configFile);

  // Read the configuration file
  let fileContents: string;
  try {
    fileContents = await fs.readFile(fullPath, "utf8");
  } catch (error) {
    throw new Error(`Failed to open config file: '${configFile}': ${error}`);
  }

  // Parse the TOML string into a Config object
  cachedConfig = parseConfig(fileContents);
  return cachedConfig;
}

/**
 * Parses a TOML string into a Config object.
 * @param {string} config The TOML configuration string.
 * @returns {Config} The parsed configuration object.
 */
function parseConfig(config: string): Config {
  try {
    return toml.parse(config) as Config;
  } catch (error) {
    throw new Error("Invalid TOML format in config string");
  }
}
