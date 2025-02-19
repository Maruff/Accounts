// Model for the coa_master table
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct CoaMaster {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub account_type: AccountType, // Use an enum
    pub parent_id: Option<i32>,
    pub currency_code: Option<String>,
    pub status: Option<Status>, // Use an enum
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Type)]
#[sqlx(type_name = "account_type", rename_all = "lowercase")]
pub enum AccountType {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Type)]
#[sqlx(type_name = "status", rename_all = "lowercase")]
pub enum Status {
    Active,
    Inactive,
}