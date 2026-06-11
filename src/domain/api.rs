#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct APIResponse<T: serde::Serialize> {
    pub code: i32,
    pub data: Option<T>,
    pub message: String,
}
