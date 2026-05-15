use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "id", rename_all = "kebab-case")]
pub enum PluginConfig {
    #[serde(rename = "auto-router")]
    AutoRouter {
        #[serde(skip_serializing_if = "Option::is_none")]
        enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        allowed_models: Option<Vec<String>>,
    },
    #[serde(rename = "moderation")]
    Moderation,
    #[serde(rename = "web")]
    Web {
        #[serde(skip_serializing_if = "Option::is_none")]
        enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max_results: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        search_prompt: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        engine: Option<String>,
    },
    #[serde(rename = "file-parser")]
    FileParser {
        #[serde(skip_serializing_if = "Option::is_none")]
        enabled: Option<bool>,
    },
    #[serde(rename = "response-healing")]
    ResponseHealing {
        #[serde(skip_serializing_if = "Option::is_none")]
        enabled: Option<bool>,
    },
}
