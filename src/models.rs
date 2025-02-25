use sqlx::FromRow;

#[derive(FromRow)]
pub struct MpesaTransaction {
    pub id: uuid::Uuid,
    pub transaction_type: String,
    pub amount: f64,
    pub phone_number: String,
    pub account_reference: String,
    pub transaction_desc: String,
    pub merchant_request_id: String,
    pub checkout_request_id: String,
    pub response_code: String,
    pub response_description: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(FromRow)]
pub struct MpesaCallback {
    pub id: uuid::Uuid,
    pub merchant_request_id: String,
    pub checkout_request_id: String,
    pub result_code: String,
    pub result_desc: String,
    pub amount: f64,
    pub mpesa_receipt_number: String,
    pub transaction_date: String,
    pub phone_number: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(FromRow)]
pub struct C2bCallback {
    pub id: uuid::Uuid,
    pub transaction_type: String,
    pub trans_id: String,
    pub trans_time: String,
    pub trans_amount: f64,
    pub business_short_code: String,
    pub bill_ref_number: String,
    pub invoice_number: String,
    pub org_account_balance: f64,
    pub third_party_trans_id: String,
    pub msisdn: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub created_at: chrono::NaiveDateTime,
}

pub async fn create_tables(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS mpesa_transactions (
            id UUID PRIMARY KEY,
            transaction_type TEXT NOT NULL,
            amount FLOAT NOT NULL,
            phone_number TEXT NOT NULL,
            account_reference TEXT NOT NULL,
            transaction_desc TEXT NOT NULL,
            merchant_request_id TEXT NOT NULL,
            checkout_request_id TEXT NOT NULL,
            response_code TEXT NOT NULL,
            response_description TEXT NOT NULL,
            created_at TIMESTAMP NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS mpesa_callbacks (
            id UUID PRIMARY KEY,
            merchant_request_id TEXT NOT NULL,
            checkout_request_id TEXT NOT NULL,
            result_code TEXT NOT NULL,
            result_desc TEXT NOT NULL,
            amount FLOAT NOT NULL,
            mpesa_receipt_number TEXT NOT NULL,
            transaction_date TEXT NOT NULL,
            phone_number TEXT NOT NULL,
            created_at TIMESTAMP NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS c2b_callbacks (
            id UUID PRIMARY KEY,
            transaction_type TEXT NOT NULL,
            trans_id TEXT NOT NULL,
            trans_time TEXT NOT NULL,
            trans_amount FLOAT NOT NULL,
            business_short_code TEXT NOT NULL,
            bill_ref_number TEXT NOT NULL,
            invoice_number TEXT NOT NULL,
            org_account_balance FLOAT NOT NULL,
            third_party_trans_id TEXT NOT NULL,
            msisdn TEXT NOT NULL,
            first_name TEXT NOT NULL,
            middle_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            created_at TIMESTAMP NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
