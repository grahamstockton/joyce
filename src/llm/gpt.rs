use anyhow::Result;
use async_openai_wasm::{
    config::OpenAIConfig,
    types::{ChatCompletionRequestSystemMessageArgs, CreateChatCompletionRequestArgs},
    Client,
};
use axum::async_trait;

use super::llm_wrapper::Llm;

pub struct GptLlm {
    client: Client<OpenAIConfig>,
}

impl GptLlm {
    pub fn init() -> Result<impl Llm, anyhow::Error> {
        Ok(Self {
            client: Client::new(),
        })
    }
}

#[async_trait]
impl Llm for GptLlm {
    async fn get_output(&self, difficulty: i32, input: String) -> Result<String> {
        let request = CreateChatCompletionRequestArgs::default()
            .model("gpt-4.1-mini")
            .messages([ChatCompletionRequestSystemMessageArgs::default()
                .content(format_prompt_string(difficulty, input))
                .build()?
                .into()])
            .max_tokens(4000_u32)
            .build()?;

        let response = self.client.chat().create(request).await.unwrap();

        Ok(format!(
            "{:?}",
            response
                .choices
                .first()
                .unwrap()
                .message
                .content
                .as_ref()
                .unwrap()
                .clone()
        ))
    }
}

fn format_prompt_string(difficulty: i32, input: String) -> String {
    format!(
        "Rephrase the following text in a grade {} reading level: {}",
        difficulty, input
    )
}
