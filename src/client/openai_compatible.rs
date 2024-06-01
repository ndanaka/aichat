use super::{
    openai::*, ChatCompletionsData, Client, ExtraConfig, Model, ModelData, ModelPatches,
    OpenAICompatibleClient, PromptAction, PromptKind, OPENAI_COMPATIBLE_PLATFORMS,
};

use anyhow::Result;
use reqwest::{Client as ReqwestClient, RequestBuilder};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct OpenAICompatibleConfig {
    pub name: Option<String>,
    pub api_base: Option<String>,
    pub api_key: Option<String>,
    pub chat_endpoint: Option<String>,
    #[serde(default)]
    pub models: Vec<ModelData>,
    pub patches: Option<ModelPatches>,
    pub extra: Option<ExtraConfig>,
}

impl OpenAICompatibleClient {
    config_get_fn!(api_base, get_api_base);
    config_get_fn!(api_key, get_api_key);

    pub const PROMPTS: [PromptAction<'static>; 5] = [
        ("name", "Platform Name:", true, PromptKind::String),
        ("api_base", "API Base:", true, PromptKind::String),
        ("api_key", "API Key:", false, PromptKind::String),
        ("models[].name", "Model Name:", true, PromptKind::String),
        (
            "models[].max_input_tokens",
            "Max Input Tokens:",
            false,
            PromptKind::Integer,
        ),
    ];

    fn chat_completions_builder(
        &self,
        client: &ReqwestClient,
        data: ChatCompletionsData,
    ) -> Result<RequestBuilder> {
        let api_base = match self.get_api_base() {
            Ok(v) => v,
            Err(err) => {
                match OPENAI_COMPATIBLE_PLATFORMS
                    .into_iter()
                    .find_map(|(name, api_base)| {
                        if name == self.model.client_name() {
                            Some(api_base.to_string())
                        } else {
                            None
                        }
                    }) {
                    Some(v) => v,
                    None => return Err(err),
                }
            }
        };
        let api_key = self.get_api_key().ok();

        let mut body = openai_build_chat_completions_body(data, &self.model);
        self.patch_request_body(&mut body);

        let chat_endpoint = self
            .config
            .chat_endpoint
            .as_deref()
            .unwrap_or("/chat/completions");

        let url = format!("{api_base}{chat_endpoint}");

        debug!("OpenAICompatible Request: {url} {body}");

        let mut builder = client.post(url).json(&body);
        if let Some(api_key) = api_key {
            builder = builder.bearer_auth(api_key);
        }

        Ok(builder)
    }
}

impl_client_trait!(
    OpenAICompatibleClient,
    crate::client::openai::openai_chat_completions,
    crate::client::openai::openai_chat_completions_streaming
);
