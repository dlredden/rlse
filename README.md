# rlse

Rlse (Release) is an open source library, written in Rust, that provides a simple efficient way to keep your feature flag definitions with your code making them a first class code citizen.

## Installation

### Install npm package

```sh
$ npm install rlse -S
```

### Create a config file

Create a new file in the root of your project called `rlse.toml`.

An example config file:

```toml
[features]
testFeature1 = { environments = ['dev', 'test']}
testFeature2 = { environments = ['test']}
testFeature3 = { environments = ['dev', 'uat', 'test', 'prod']}
```

You can name this file anything you want, just set the `RLSE_CONFIG` environment variable to the name of the file if you name it something other than `rlse.toml`.

**_Be sure to include `rlse.toml` in your build process so it deploys with your application._**

## Features

The `[features]` section is the where the magic happens. The simplest definition looks like this.

```toml
uniqueFriendlyFeatureName = { environments = ['dev']}
```

This simply states that `uniqueFriendlyFeatureName` is enabled in the `dev` environment, environment is inferred from `APP_ENV` or passed to the test function.

## Usage

Wrap your abstraction in a test.

```javascript
import { is_enabled } from 'rlse';

// Will look for APP_ENV to be set
// Defaults to 'dev' environment if not set
if (is_enabled('uniqueFriendlyFeatureName')) { ... }

// Or you can pass in an environment name as the second parameter
if (is_enabled('uniqueFriendlyFeatureName', 'dev')) { ... }
```

**_`is_enabled()` will return `false` unless it finds a feature/environment combiantion defined in the config file._**
