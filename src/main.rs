extern crate core;

mod setup;
mod entities;

use dotenv::dotenv;
use rocket::*;
use rocket::serde::json::Json;
use sea_orm::{DatabaseConnection, DbConn, EntityTrait};
use crate::entities::prelude::Account;
use crate::setup::set_up_db;

#[get("/")]
fn index() -> &'static str {
    "Hello World!"
}

#[get("/accounts")]
async fn accounts(db: &State<DbConn>) -> Json<Vec<String>> {
    let db = db as &DatabaseConnection;

    let account_names = Account::find()
        .all(db)
        .await
        .unwrap()
        .into_iter()
        .map(|x| x.name)
        .collect::<Vec<String>>();

    Json(account_names)
}

#[launch]
async fn rocket() -> Rocket<Build> {
    dotenv().ok();
    let db = match set_up_db().await {
        Ok(db) => db,
        Err(error) => panic!("{error}")
    };

    rocket::build()
        .manage(db)
        .mount("/",
               routes![
                   index,
                   accounts,
               ])
}