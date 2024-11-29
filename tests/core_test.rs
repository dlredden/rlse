#[cfg(test)]
mod tests {
  use std::collections::HashMap;
  use rlse::config::{Config, Feature, Issue, Sources};
  use rlse::feature::{is_feature_enabled, get_feature_issue_source};

    #[test]
    fn test_is_feature_enabled() {
        let mut features = HashMap::new();
        features.insert(
            "testFeature".to_string(),
            Feature {
                environments: vec!["dev".to_string(), "prod".to_string()],
                issue: None,
            },
        );

        let config = Config {
            features,
            sources: Sources {
                default: "https://default.com".to_string(),
                jira: Some("https://jira.example.com".to_string()),
                bitbucket: Some("https://bitbucket.example.com".to_string()),
                github: Some("https://github.com".to_string()),
            },
        };

        assert!(is_feature_enabled("testFeature", "dev", &config));
        assert!(!is_feature_enabled("testFeature", "staging", &config));
    }

    #[test]
    fn test_get_feature_issue_source() {
        let mut features = HashMap::new();
        features.insert(
            "testFeature".to_string(),
            Feature {
                environments: vec!["dev".to_string(), "prod".to_string()],
                issue: Some(Issue {
                    id: "123".to_string(),
                    source: Some("jira".to_string()),
                }),
            },
        );

        let config = Config {
            features,
            sources: Sources {
                default: "https://default.com".to_string(),
                jira: Some("https://jira.example.com".to_string()),
                bitbucket: Some("https://bitbucket.example.com".to_string()),
                github: Some("https://github.com".to_string()),
            },
        };

        assert_eq!(
            get_feature_issue_source("testFeature", &config),
            Some("https://jira.example.com/123".to_string())
        );
    }

    #[test]
    fn test_get_feature_issue_source_with_default() {
        let mut features = HashMap::new();
        features.insert(
            "testFeature".to_string(),
            Feature {
                environments: vec!["dev".to_string(), "prod".to_string()],
                issue: Some(Issue {
                    id: "456".to_string(),
                    source: None,
                }),
            },
        );

        let config = Config {
            features,
            sources: Sources {
                default: "https://default.com".to_string(),
                jira: Some("https://jira.example.com".to_string()),
                bitbucket: Some("https://bitbucket.example.com".to_string()),
                github: Some("https://github.com".to_string()),
            },
        };

        assert_eq!(
            get_feature_issue_source("testFeature", &config),
            Some("https://default.com/456".to_string())
        );
    }
}