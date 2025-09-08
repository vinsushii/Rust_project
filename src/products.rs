#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub(crate) id: u32,
    pub(crate) product: String,
    pub(crate) price: f64,
}