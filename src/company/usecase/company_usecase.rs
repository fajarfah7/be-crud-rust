use chrono::Utc;
use uuid::Uuid;

use crate::company::domain::company::Company;
use crate::company::repository::company_repository::CompanyRepository;
use crate::company::usecase;
use crate::request::pagination::PaginationRequest;

pub struct CompanyUsecase<R: CompanyRepository> {
    repo: R,
}

pub enum CompanyUsecaseError {
    EmailAlreadyExist,
    CodeAlreadyExist,
    NotFound,
    DatabaseError,
}

impl<R: CompanyRepository> CompanyUsecase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create_company(
        &self,
        name: String,
        email: String,
        code: String,
        phone_number: Option<String>,
        address: Option<String>,
    ) -> Result<Company, CompanyUsecaseError> {
        let is_company_email_exist = self
            .repo
            .check_existing_company_email(&email, None)
            .await
            .map_err(|_| CompanyUsecaseError::DatabaseError)?;
        if is_company_email_exist {
            return Err(CompanyUsecaseError::EmailAlreadyExist);
        }

        let is_company_code_exist = self
            .repo
            .check_existing_company_code(&code, None)
            .await
            .map_err(|_| CompanyUsecaseError::DatabaseError)?;
        if is_company_code_exist {
            return Err(CompanyUsecaseError::CodeAlreadyExist);
        }

        let company = Company {
            id: Uuid::new_v4(),
            name,
            email,
            code,
            phone_number,
            address,
            created_at: Utc::now(),
        };

        self.repo
            .create_company(company)
            .await
            .map_err(|_| CompanyUsecaseError::DatabaseError)
    }

    pub async fn update_company(
        &self,
        id: Uuid,
        name: String,
        email: String,
        code: String,
        phone_number: Option<String>,
        address: Option<String>,
    ) -> Result<Company, CompanyUsecaseError> {
        let is_company_email_exist = self
            .repo
            .check_existing_company_email(&email, Some(&id))
            .await
            .map_err(|_| CompanyUsecaseError::DatabaseError)?;
        if is_company_email_exist {
            return Err(CompanyUsecaseError::EmailAlreadyExist);
        }

        let is_company_code_exist = self
            .repo
            .check_existing_company_code(&code, Some(&id))
            .await
            .map_err(|_| CompanyUsecaseError::DatabaseError)?;
        if is_company_code_exist {
            return Err(CompanyUsecaseError::CodeAlreadyExist);
        }

        let get_company = self
            .repo
            .get_company_by_id(&id)
            .await
            .map_err(|_| CompanyUsecaseError::DatabaseError)?;
        if get_company.is_none() {
            return Err(CompanyUsecaseError::NotFound);
        }

        let mut company = get_company.unwrap();
        company.name = name;
        company.code = code;
        company.email = email;
        company.phone_number = phone_number;
        company.address = address;

        self.repo
            .update_company(company)
            .await
            .map_err(|_| CompanyUsecaseError::DatabaseError)
    }

    pub async fn delete_company(&self, id: Uuid) -> Result<(), CompanyUsecaseError> {
        let get_company = self
            .repo
            .get_company_by_id(&id)
            .await
            .map_err(|_| CompanyUsecaseError::DatabaseError)?;

        if get_company.is_none() {
            return Err(CompanyUsecaseError::NotFound);
        }
        self.repo
            .delete_company(&id)
            .await
            .map_err(|_| CompanyUsecaseError::DatabaseError)
    }

    pub async fn list_company(
        &self,
        query: &PaginationRequest,
    ) -> Result<Vec<Company>, CompanyUsecaseError> {
        let total_company = self
            .repo
            .count_all_companies(&query)
            .await
            .map_err(|_| CompanyUsecaseError::DatabaseError);

        self.repo
            .find_all_companies(&query)
            .await
            .map_err(|_| CompanyUsecaseError::DatabaseError)
    }
}
