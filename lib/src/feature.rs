use crate::config::Config;

/// Checks if a feature is enabled for a given environment.
///
/// This function also considers the feature's associated issue and its source if present.
///
/// # Arguments
/// - `feature_name`: The name of the feature to check.
/// - `env`: The environment to check against (e.g., "dev", "prod").
/// - `config`: The application configuration containing features and sources.
///
/// # Returns
/// - `true` if the feature is enabled for the given environment.
/// - `false` otherwise.
pub fn is_feature_enabled(feature_name: &str, env: &str, config: &Config) -> bool {
    // Look up the feature by name
    if let Some(feature) = config.features.get(feature_name) {
        // Check if the environment is enabled for this feature
        if feature.environments.contains(&env.to_string()) {
            return true;
        }
    }
    false
}

/// Gets the issue source URL for a feature, if it exists.
///
/// # Arguments
/// - `feature_name`: The name of the feature.
/// - `config`: The application configuration containing features and sources.
///
/// # Returns
/// - `Some(String)`: The URL of the issue's source if available.
/// - `None`: If no issue or source is defined for the feature.
#[allow(dead_code)]
pub fn get_feature_issue_source(feature_name: &str, config: &Config) -> Option<String> {
    // Look up the feature by name
    if let Some(feature) = config.features.get(feature_name) {
        if let Some(issue) = &feature.issue {
            // Determine the source for the issue
            let source_url: Option<String> = match &issue.source {
                Some(source) => match source.as_str() {
                    "jira" => config.sources.jira.clone(),
                    "bitbucket" => config.sources.bitbucket.clone(),
                    "github" => config.sources.github.clone(),
                    _ => None, // Unknown source
                },
                None => Some(config.sources.default.clone()), // Fallback to default source
            };

            // Return the source URL with the issue ID appended, if available
            if let Some(url) = source_url {
                return Some(format!("{}/{}", url, issue.id));
            }
        }
    }
    None
}