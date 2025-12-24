use sqlx::{Postgres, QueryBuilder};

pub fn apply_search_filter(
    qb: &mut QueryBuilder<Postgres>,
    search: &Option<String>,
) {
    if let Some(s) = search {
        qb.push(" WHERE (")
          .push(" name ILIKE ")
          .push_bind(format!("%{s}%"))
          .push(" OR code ILIKE ")
          .push_bind(format!("%{s}%"))
          .push(")");
    }
}
