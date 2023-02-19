use std::env;
use sea_orm::*;

pub async fn set_up_db() -> Result<DbConn, DbErr>{
    let db_url:String = env::var("DB_URL").expect("DB_URL must be set.");
    let db_name:String = env::var("DB_NAME").expect("DB_NAME must be set.");

    let db = Database::connect(&db_url).await?;
    let db = match db.get_database_backend() {
        DatabaseBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", &db_name),
            )).await?;

            let url = format!("{}/{}", &db_url, &db_name);
            Database::connect(&url).await?
        }
        _ => panic!("Please check your database is MySQL.")
    };

    Ok(db)
}