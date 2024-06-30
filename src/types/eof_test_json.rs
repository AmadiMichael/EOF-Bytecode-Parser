use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub valid_invalid: Tests,
}

#[derive(Debug, Deserialize)]
pub struct Tests {
    pub _info: Info,
    pub vectors: HashMap<String, Vectors>,
}

#[derive(Debug, Deserialize)]
pub struct Vectors {
    pub code: String,
    pub results: Results,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Results {
    pub prague: Prague,
}

#[derive(Debug, Deserialize)]
pub struct Prague {
    pub result: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[allow(dead_code)]
pub struct Info {
    pub comment: String,
}
