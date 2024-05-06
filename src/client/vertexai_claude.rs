use super::claude::{claude_build_body, claude_send_message, claude_send_message_streaming};
use super::vertexai::fetch_gcloud_access_token;
use super::{
    Client, CompletionDetails, ExtraConfig, Model, ModelConfig, PromptAction, PromptKind, SendData,
    SseHandler, VertexAIClaudeClient,
};

use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use chrono::{Duration, Utc};
use reqwest::{Client as ReqwestClient, RequestBuilder};
use serde::Deserialize;

static mut ACCESS_TOKEN: (String, i64) = (String::new(), 0); // safe under linear operation

#[derive(Debug, Clone, Deserialize, Default)]
pub struct VertexAIClaudeConfig {
    pub name: Option<String>,
    pub project_id: Option<String>,
    pub location: Option<String>,
    pub adc_file: Option<String>,
    #[serde(default)]
    pub models: Vec<ModelConfig>,
    pub extra: Option<ExtraConfig>,
}

impl VertexAIClaudeClient {
    config_get_fn!(project_id, get_project_id);
    config_get_fn!(location, get_location);

    pub const PROMPTS: [PromptAction<'static>; 2] = [
        ("project_id", "Project ID", true, PromptKind::String),
        ("location", "Location", true, PromptKind::String),
    ];

    fn request_builder(&self, client: &ReqwestClient, data: SendData) -> Result<RequestBuilder> {
        let project_id = self.get_project_id()?;
        let location = self.get_location()?;

        let base_url = format!("https://{location}-aiplatform.googleapis.com/v1/projects/{project_id}/locations/{location}/publishers");
        let url = format!(
            "{base_url}/anthropic/models/{}:streamRawPredict",
            self.model.name
        );

        let mut body = claude_build_body(data, &self.model)?;
        if let Some(body_obj) = body.as_object_mut() {
            body_obj.remove("model");
        }
        body["anthropic_version"] = "vertex-2023-10-16".into();

        debug!("VertexAIClaude Request: {url} {body}");

        let builder = client
            .post(url)
            .bearer_auth(unsafe { &ACCESS_TOKEN.0 })
            .json(&body);

        Ok(builder)
    }
}

#[async_trait]
impl Client for VertexAIClaudeClient {
    client_common_fns!();

    async fn send_message_inner(
        &self,
        client: &ReqwestClient,
        data: SendData,
    ) -> Result<(String, CompletionDetails)> {
        prepare_access_token(client, &self.config.adc_file).await?;
        let builder = self.request_builder(client, data)?;
        claude_send_message(builder).await
    }

    async fn send_message_streaming_inner(
        &self,
        client: &ReqwestClient,
        handler: &mut SseHandler,
        data: SendData,
    ) -> Result<()> {
        prepare_access_token(client, &self.config.adc_file).await?;
        let builder = self.request_builder(client, data)?;
        claude_send_message_streaming(builder, handler).await
    }
}

async fn prepare_access_token(client: &reqwest::Client, adc_file: &Option<String>) -> Result<()> {
    if unsafe { ACCESS_TOKEN.0.is_empty() || Utc::now().timestamp() > ACCESS_TOKEN.1 } {
        let (token, expires_in) = fetch_gcloud_access_token(client, adc_file)
            .await
            .with_context(|| "Failed to fetch access token")?;
        let expires_at = Utc::now()
            + Duration::try_seconds(expires_in)
                .ok_or_else(|| anyhow!("Failed to parse expires_in of access_token"))?;
        unsafe { ACCESS_TOKEN = (token, expires_at.timestamp()) };
    }
    Ok(())
}
