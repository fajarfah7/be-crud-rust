use async_trait::async_trait;
use uuid::Uuid;
use crate::company::domain::company::Company;
use crate::app_request::pagination::PaginationRequest;

#[async_trait]
pub trait CompanyRepository: Send + Sync {
    async fn get_company_by_id(&self, id: &Uuid) -> Result<Option<Company>, sqlx::Error>;
    async fn count_all_companies(&self, query: &PaginationRequest, ) -> Result<i64, sqlx::Error>;
    async fn find_all_companies(&self, query: &PaginationRequest) -> Result<Vec<Company>, sqlx::Error>;
    async fn check_existing_company_email(&self, email: &str, id: Option<&Uuid>) -> Result<bool, sqlx::Error>;
    async fn check_existing_company_code(&self, code: &str, id: Option<&Uuid>) -> Result<bool, sqlx::Error>;
    async fn create_company(&self, company: Company) -> Result<Company, sqlx::Error>;
    async fn update_company(&self, company: Company) -> Result<Company, sqlx::Error>;
    async fn delete_company(&self, id: &Uuid) -> Result<(), sqlx::Error>;
    // async fn delete_company(&self, id: Uuid) -> Result<(), sqlx::Error>;
}