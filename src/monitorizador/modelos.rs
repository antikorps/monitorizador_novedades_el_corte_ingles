use serde::Deserialize;

pub type ProductosECI = Vec<ProductoECI>;
pub type DataLayerResponse = Vec<DataLayer>;

#[derive(Clone, Deserialize)]
pub struct DataLayer {
    pub products: Vec<ProductoECI>,
}

#[derive(Clone, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct ProductoECI {
    pub name: String,
    pub price: Price,
    // #[serde(rename = "code_a")]
    pub code_a: String,
    // pub gtin: String,
    pub hierarchy: Vec<String>,
    // pub id: String,
    pub notificar: Option<bool>,
    pub url: Option<String>,
}

#[derive(Clone, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct Price {
    pub currency: Option<String>,
    // pub discount: Option<f64>,
    // #[serde(rename = "discount_percent")]
    pub discount_percent: Option<f64>,
    // #[serde(rename = "f_price")]
    pub f_price: Option<f64>,
    // #[serde(rename = "o_price")]
    pub o_price: Option<f64>,
}
