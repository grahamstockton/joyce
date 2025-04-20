use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;

use super::gpt::GptLlm;

#[async_trait]
pub trait Llm {
    async fn get_output(&self, difficulty: i32, input: String) -> Result<String>;
}

// Serves as a wrapper for our various LLMs
// Should make it easier to switch in the future
#[derive(Clone)]
pub struct LlmWrapper {
    llm: Arc<dyn Llm + Send + Sync>,
}

impl LlmWrapper {
    pub fn new(llm_type: LlmType) -> LlmWrapper {
        LlmWrapper {
            llm: match llm_type {
                LlmType::Gpt => Arc::new(GptLlm::init().unwrap()),
            },
        }
    }

    pub async fn get_output(&self, difficulty: i32, input: String) -> Result<String> {
        self.llm.get_output(difficulty, input).await
    }
}

pub enum LlmType {
    Gpt,
}
