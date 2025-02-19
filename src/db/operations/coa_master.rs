// Operations for the coa_master table
use sqlx::PgPool;
use crate::db::models::CoaMaster;

pub async fn get_all_coa_masters(pool: &PgPool) -> Result<Vec<CoaMaster>, sqlx::Error> {
    let coa_masters = sqlx::query_as::<_, CoaMaster>(
        "SELECT * FROM coa_master"
    )
    .fetch_all(pool)
    .await?;

    Ok(coa_masters)
}

pub async fn create_coa_master(pool: &PgPool, new_coa: &CoaMaster) -> Result<CoaMaster, sqlx::Error> {
    let created_coa = sqlx::query_as::<_, CoaMaster>(
        "INSERT INTO coa_master (name, code, account_type, parent_id, currency_code, status) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(&new_coa.name)
    .bind(&new_coa.code)
    .bind(new_coa.account_type)
    .bind(new_coa.parent_id)
    .bind(&new_coa.currency_code)
    .bind(new_coa.status)
    .fetch_one(pool)
    .await?;

    Ok(created_coa)
}
// Add update and delete functions as needed.