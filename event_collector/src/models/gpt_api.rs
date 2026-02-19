use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ResponseRequest {
    pub model: String,
    pub input: Vec<Message>,
    pub max_output_tokens: u32,
    pub temperature: f32, // how deterministic
}

#[derive(Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct ResponseBody {
    pub output: Vec<OutputBlock>,
}

#[derive(Deserialize, Debug)]
pub struct OutputBlock {
    pub content: Option<Vec<OutputContent>>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OutputContent {
    OutputText { text: String },
}
