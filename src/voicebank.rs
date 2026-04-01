use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Voicebank {
    pub name: String,
    pub creator: String,
    pub bank_type: String, // e.g., VCV, CV, VCCV
    pub language: String,
    pub download_link: String,
    pub description: String,
}

impl Voicebank {
    pub fn new() -> Self {
        Self {
            name: "Novo Voicebank".to_string(),
            ..Default::default()
        }
    }
}
