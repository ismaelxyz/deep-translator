#[derive(Debug, Default, Eq, PartialEq, Clone, serde::Deserialize, serde::Serialize)]

pub struct Libre {
    pub api_key: String,
    pub url: String,
    pub alternatives: usize
}