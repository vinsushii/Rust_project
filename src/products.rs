use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub(crate) id: u32,
    pub(crate) product: String,
    pub(crate) quantity: f64.
    pub(crate) price: f64,
}
