use serde::Deserialize;
use sqlx::PgPool;
use tracing::{info, error};
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Deserialize)]
pub struct MpesaCallbackData {
    pub merchant_request_id: String,
    pub checkout_request_id: String,
    pub result_code: String,
    pub result_desc: String,
    pub amount: f64,
    pub mpesa_receipt_number: String,
    pub transaction_date: String,
    pub phone_number: String,
}

#[derive(Debug, Deserialize)]
pub struct C2bCallbackData {
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
}

pub async fn handle_callback(
    pool: &PgPool,
    callback_data: MpesaCallbackData,
) -> Result<(), sqlx::Error> {
    info!("Received M-PESA callback: {:?}", callback_data);

    let result = sqlx::query(
        r#"
        INSERT INTO mpesa_callbacks (
            id, merchant_request_id, checkout_request_id, result_code, result_desc,
            amount, mpesa_receipt_number, transaction_date, phone_number, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(callback_data.merchant_request_id)
    .bind(callback_data.checkout_request_id)
    .bind(callback_data.result_code)
    .bind(callback_data.result_desc)
    .bind(callback_data.amount)
    .bind(callback_data.mpesa_receipt_number)
    .bind(callback_data.transaction_date)
    .bind(callback_data.phone_number)
    .bind(Utc::now().naive_utc())
    .execute(pool)
    .await;

    match result {
        Ok(_) => {
            info!("Callback data saved successfully.");
            Ok(())
        }
        Err(e) => {
            error!("Failed to save callback data: {:?}", e);
            Err(e)
        }
    }
}

pub async fn handle_c2b_callback(
    pool: &PgPool,
    callback_data: C2bCallbackData,
) -> Result<(), sqlx::Error> {
    info!("Received C2B callback: {:?}", callback_data);

    let result = sqlx::query(
        r#"
        INSERT INTO c2b_callbacks (
            id, transaction_type, trans_id, trans_time, trans_amount,
            business_short_code, bill_ref_number, invoice_number,
            org_account_balance, third_party_trans_id, msisdn,
            first_name, middle_name, last_name, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(callback_data.transaction_type)
    .bind(callback_data.trans_id)
    .bind(callback_data.trans_time)
    .bind(callback_data.trans_amount)
    .bind(callback_data.business_short_code)
    .bind(callback_data.bill_ref_number)
    .bind(callback_data.invoice_number)
    .bind(callback_data.org_account_balance)
    .bind(callback_data.third_party_trans_id)
    .bind(callback_data.msisdn)
    .bind(callback_data.first_name)
    .bind(callback_data.middle_name)
    .bind(callback_data.last_name)
    .bind(Utc::now().naive_utc())
    .execute(pool)
    .await;

    match result {
        Ok(_) => {
            info!("C2B callback data saved successfully.");
            Ok(())
        }
        Err(e) => {
            error!("Failed to save C2B callback data: {:?}", e);
            Err(e)
        }
    }
}
