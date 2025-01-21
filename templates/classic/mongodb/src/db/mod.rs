use std::sync::OnceLock;

use crate::models::User;
use mongodb::{
    bson::{doc, Document},
    options::IndexOptions,
    Client, IndexModel,
};

use crate::config::DbConfig;

pub static MONGODB_CLIENT: OnceLock<Client> = OnceLock::new();

pub async fn init(config: &DbConfig) {
    let client = Client::with_uri_str(&config.url)
        .await
        .expect("failed to connect");
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { "username": 1 })
        .options(options)
        .build();
    client
        .database(&config.database)
        .collection::<User>(&config.collection)
        .create_index(model)
        .await
        .expect("creating an index should succeed");
    MONGODB_CLIENT
        .set(client)
        .expect("seaorm pool should be set");
}

pub fn client() -> &'static Client {
    MONGODB_CLIENT.get().expect("seaorm pool should set")
}

pub fn users() -> mongodb::Collection<Document> {
    let config = &crate::config::get().db;
    client()
        .database(&config.database)
        .collection::<Document>(&config.collection)
}
