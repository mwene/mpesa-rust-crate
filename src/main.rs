use mpesa_rs::{MpesaCrate, client::MpesaClient};
use mpesa_rs::callback::{MpesaCallbackData, C2bCallbackData, handle_callback, handle_c2b_callback};
use warp::Filter;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the crate
    let mpesa_crate = MpesaCrate::new().await?;
    let db_pool = mpesa_crate.db_pool.clone();
    let callback_port = mpesa_crate.settings.mpesa.callback_port;

    // Define the STK callback route
    let stk_callback_route = warp::path("stk_callback")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |callback_data: MpesaCallbackData| {
            let pool = db_pool.clone();
            async move {
                handle_callback(&pool, callback_data)
                    .await
                    .map_err(|e| warp::reject::custom(e))
            }
        });

    // Define the C2B callback route
    let c2b_callback_route = warp::path("c2b_callback")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |callback_data: C2bCallbackData| {
            let pool = db_pool.clone();
            async move {
                handle_c2b_callback(&pool, callback_data)
                    .await
                    .map_err(|e| warp::reject::custom(e))
            }
        });

    // Combine routes
    let routes = stk_callback_route.or(c2b_callback_route);

    // Start the server
    let server_address = format!("127.0.0.1:{}", callback_port);
    println!("Listening for callbacks on http://{}", server_address);

    warp::serve(routes)
        .run(server_address.parse().unwrap())
        .await;

    Ok(())
}
