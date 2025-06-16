use serde::Deserialize;

#[derive(Deserialize)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub enum TransformCase {
    #[serde(rename(deserialize = "uppercase"))]
    UpperCase,
    #[serde(rename(deserialize = "lowercase"))]
    LowerCase,
}

#[derive(Deserialize)]
pub struct TransformInput {
    pub transform: TransformCase,
    pub html: String,
}
