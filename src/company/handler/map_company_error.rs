use crate::company::handler::types::ProcessCompanyRequest;
use crate::company::usecase::company_usecase::CompanyUsecaseError;
use crate::helper::helper::is_option_has_string_value;
use crate::response::error::ResponseError;

pub fn map_usecase_company_error(err: CompanyUsecaseError) -> ResponseError {
    match err {
        CompanyUsecaseError::EmailAlreadyExist => {
            ResponseError::BadRequest("email already exist".into())
        }
        CompanyUsecaseError::CodeAlreadyExist => {
            ResponseError::BadRequest("code already exist".into())
        }
        CompanyUsecaseError::NotFound => {
            ResponseError::NotFound("data not found".into())
        }
        CompanyUsecaseError::DatabaseError => ResponseError::DatabaseError,
    }
}

pub fn validate_company_input(req: &ProcessCompanyRequest) -> Result<(), ResponseError> {
    if req.name == "" {
        return Err(ResponseError::BadRequest("Name is required".into()));
    }
    if req.email == "" {
        return Err(ResponseError::BadRequest("Email is required".into()));
    }
    if req.code == "" {
        return Err(ResponseError::BadRequest("Code is required".into()));
    }
    if !is_option_has_string_value(&req.phone_number) {
        return Err(ResponseError::BadRequest("Phone number is required".into()));
    }
    if !is_option_has_string_value(&req.address) {
        return Err(ResponseError::BadRequest("Address is required".into()))
    }
    Ok(())
}
