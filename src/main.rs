extern crate core;

mod setup;
mod entities;

use dotenv::dotenv;
use rocket::*;
use rocket::http::Status;
use rocket::serde::json::Json;
use sea_orm::{ActiveValue, DatabaseConnection, DbConn, EntityTrait};
use sea_orm::sea_query::IdenList;
use crate::entities::account;
use crate::entities::account::Model;
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

#[post("/account", data="<data>")]
async fn createAccount(data: Json<account::InputData>, db: &State<DbConn>) -> Result<Json<i32>, Status> {
    let db = db as &DatabaseConnection;

    let account = account::ActiveModel {
        name: ActiveValue::Set(data.name.to_string()),
        email: ActiveValue::Set(data.email.to_string()),
        password:  ActiveValue::Set(pwhash::bcrypt::hash(data.password.to_string()).unwrap()),
        ..Default::default()
    };

    let res = Account::insert(account)
        .exec(db)
        .await
        .expect("could not insert account.");

    Ok(Json(res.last_insert_id))
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
                   createAccount,
               ])
}