use async_trait::async_trait;
use sqlx::{PgPool, QueryBuilder};
// use tracing::{debug, info};
use uuid::Uuid;

use crate::company::domain::company::Company;
use crate::company::repository::company_repository::CompanyRepository;
use crate::company::repository::helper_query::apply_search_filter;
use crate::request::pagination::PaginationRequest;

pub struct CompanyRepositorySqlx {
    pool: PgPool,
}

impl CompanyRepositorySqlx {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CompanyRepository for CompanyRepositorySqlx {
    async fn get_company_by_id(&self, id: &Uuid) -> Result<Option<Company>, sqlx::Error> {
        let company = sqlx::query_as!(
            Company,
            r#"
            SELECT id, name, email, code, address, phone_number, created_at
            FROM companies
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Some(company))
    }

    async fn check_existing_company_email(
        &self,
        email: &str,
        id: Option<&Uuid>,
    ) -> Result<bool, sqlx::Error> {
        let is_exist = match id {
            Some(id) => {
                sqlx::query_scalar!(
                    r#"
                    SELECT EXISTS (
                        SELECT 1 
                        FROM companies
                        WHERE email = $1 AND id != $2
                    )
                    "#,
                    email,
                    id
                )
                .fetch_one(&self.pool)
                .await?
            }
            None => {
                sqlx::query_scalar!(
                    r#"
                    SELECT EXISTS (
                        SELECT 1 
                        FROM companies
                        WHERE email = $1
                    )
                    "#,
                    email
                )
                .fetch_one(&self.pool)
                .await?
            }
        };

        Ok(is_exist.unwrap_or(false))
    }

    async fn check_existing_company_code(
        &self,
        code: &str,
        id: Option<&Uuid>,
    ) -> Result<bool, sqlx::Error> {
        let is_exist = match id {
            Some(id) => {
                sqlx::query_scalar!(
                    r#"
                    SELECT EXISTS (
                        SELECT 1 
                        FROM companies
                        WHERE code = $1 AND id != $2
                    )
                    "#,
                    code,
                    id,
                )
                .fetch_one(&self.pool)
                .await?
            }
            None => {
                sqlx::query_scalar!(
                    r#"
                    SELECT EXISTS (
                        SELECT 1
                        FROM companies
                        WHERE code = $1
                    )
                    "#,
                    code,
                )
                .fetch_one(&self.pool)
                .await?
            }
        };

        Ok(is_exist.unwrap_or(false))
    }

    async fn create_company(&self, company: Company) -> Result<Company, sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO companies
            (id, name, email, code, phone_number, address, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, name, email, code, phone_number, address, created_at
            "#,
            company.id,
            company.name,
            company.email,
            company.code,
            company.phone_number,
            company.address,
            company.created_at,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(company)
    }

    async fn update_company(&self, company: Company) -> Result<Company, sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE companies
            SET name = $1,
                email = $2,
                code = $3,
                phone_number = $4,
                address = $5
            WHERE id = $6
            RETURNING id, name, email, code, phone_number, address, created_at
            "#,
            company.name,
            company.email,
            company.code,
            company.phone_number,
            company.address,
            company.id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(company)
    }

    async fn delete_company(&self, id: &Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM companies WHERE id = $1"#, id,)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn count_all_companies(&self, query: &PaginationRequest) -> Result<i64, sqlx::Error> {
        let mut qb = QueryBuilder::new("SELECT COUNT(id)FROM companies");

        apply_search_filter(&mut qb, &query.search);
        // if let Some(s) = &query.search {
        //     qb.push(" WHERE ")
        //         .push(" ( ")
        //         .push(" name ILIKE ")
        //         .push_bind(format!("%{s}%"))
        //         .push(" OR code ILIKE ")
        //         .push_bind(format!("%{s}%"))
        //         .push(" ) ");
        // }

        let total: i64 = qb.build_query_scalar().fetch_one(&self.pool).await?;
        Ok(total)
    }

    async fn find_all_companies(
        &self,
        query: &PaginationRequest,
    ) -> Result<Vec<Company>, sqlx::Error> {
        let mut qb = QueryBuilder::new(
            "
            SELECT id, name, email, code, phone_number, address, created_at
            FROM companies
        ",
        );

        apply_search_filter(&mut qb, &query.search);
        // if let Some(s) = &query.search {
        //     qb.push(" WHERE ")
        //         .push(" ( ")
        //         .push(" name ILIKE ")
        //         .push_bind(format!("%{s}%"))
        //         .push(" OR code ILIKE ")
        //         .push_bind(format!("%{s}%"))
        //         .push(" ) ");
        // }

        if let Some(s) = query.format_sort() {
            qb.push(" ORDER BY ").push(format!("{}", s));
        }

        qb.push(" LIMIT ")
            .push_bind(query.per_page.unwrap_or(1) as i64); // FOR LIMIT MUST i64

        if let Some(o) = query.offset {
            qb.push(" OFFSET ").push_bind(o as i64); // FOR LIMIT MUST i64
        }

        let companies = qb.build_query_as::<Company>().fetch_all(&self.pool).await?;
        Ok(companies)
    }
}
