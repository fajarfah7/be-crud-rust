use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ProcessCompanyRequest {
    // pub id: Option<String>,
    pub name: String,
    pub email: String,
    pub code: String,
    pub phone_number: Option<String>,
    pub address: Option<String>,
}