use crate::company::domain::company::Company;

pub struct ListCompanyResult {
    pub data: Vec<Company>,
    pub total_data: i64,
}