# rlse

Rlse (pronounced “release”) is an open-source library that makes feature flags a first-class citizen in your codebase. With Rlse, you can easily manage feature flag definitions directly in your code, improving consistency, simplicity, and efficiency in your release process.

## Installation

### Install the npm package

Add Rlse to your project using npm:

```sh
$ npm install rlse -S
```

### Create a configuration file

Create a configuration file in the root of your project to define your feature flags. By default, Rlse looks for a file named `rlse.toml`. Here’s an example:

```toml
[features]
testFeature1 = { environments = ['dev', 'test']}
testFeature2 = { environments = ['test']}
testFeature3 = { environments = ['dev', 'uat', 'test', 'prod']}
```

If you prefer a different file name, set the `RLSE_CONFIG` environment variable to point to it. For example:

```sh
export RLSE_CONFIG=my_custom_config.toml
```

💡 Tip: Ensure the configuration file is included in your build process so it deploys with your application.

## Defining Features

The `[features]` section is where you define your feature flags. Each feature is a key-value pair, where the key is the feature name and the value specifies the environments in which it is enabled.

**Example:**

```toml
[features]
uniqueFriendlyFeatureName = { environments = ['dev', 'test']}
```

In this example:

- The uniqueFriendlyFeatureName feature is enabled only in the dev environment.
- The environment can be determined dynamically from the `APP_ENV` environment variable or explicitly passed when testing the feature flag.

## Usage

Rlse makes it easy to conditionally enable features based on your configuration. Use the is_enabled function to check whether a feature is active in a given environment.

**Example:**

```javascript
import { isFeatureEnabled } from "rlse";

// Automatically uses environment variabls: process.env.APP_ENV | 'dev' and process.env.RLSE_CONFIG | 'rlse.toml'.
if (isFeatureEnabled({ feature: "testFeatureDevTest" })) {
  // Feature-specific logic ...
}

// Explicitly specify the environment and/or config data.
// This is useful in a browser/client where bundling your toml file with your app is required.
if (isFeatureEnabled({ feature: "missingFeature", env: "dev", config })) {
  // Feature-specific logic ...
}
```

🛑 Important:

- is_enabled() will return false unless the feature/environment combination is explicitly defined in the configuration file.
- Ensure your configuration file accurately reflects your environment setup.

## Why Choose Rlse?

- Simplify Feature Management: Centralize and standardize feature flag definitions across environments.
- First-Class Code Citizen: Keep feature flags in your codebase, reducing the risk of misalignment.
