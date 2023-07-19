use {
    actix_web::HttpResponse,
    actix_web::web::{Data, Json, Path},
    uuid::Uuid,

    crate::DBPool,

    crate::wallet::*,
    crate::util::*,
};

#[get("/wallets")]
pub async fn list_wallets(pool: Data<DBPool>) -> HttpResponse {
    
    let conn  = crate::get_connection_to_pool(pool);
    let wallets: Vec<Wallet> = fetch_all_wallet(&conn);
    ResponseType::Ok(wallets).get_response()
}

#[get("/wallets/{id}")]
pub async fn get_wallet(path: Path<(String,)>, pool: Data<DBPool>) -> HttpResponse {
    let conn  = crate::get_connection_to_pool(pool);

    let wallet: Option<Wallet> = fetch_wallet_by_id(
        Uuid::parse_str(path.0.0.as_str()).unwrap(), &conn);
    match wallet {
        Some(wallet) => ResponseType::Ok(wallet).get_response(),
        None => ResponseType::NotFound(
            NotFoundMessage::new("Wallet not found.".to_string())
        ).get_response(),
    }
}

// create new wallet
#[post("/wallets")]
pub async fn create_wallet(pool: Data<DBPool>, wallet_request: Json<NewWalletRequest>) -> HttpResponse {
    let conn  = crate::get_connection_to_pool(pool);
    match create_new_wallet(wallet_request.0, &conn) {
        Ok(created_wallet) => ResponseType::Created(created_wallet).get_response(),
        Err(_) => ResponseType::NotFound(
            NotFoundMessage::new("Error creating wallet.".to_string())
        ).get_response(),
    }
}