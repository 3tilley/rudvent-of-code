use sea_orm::{DeriveEntityModel, SqlxPostgresConnector};
use shuttle_axum::ShuttleAxum;
use shuttle_secrets::SecretStore;
use rudvent::build_router;
use sqlx::PgPool;
// use sea_orm::entity::prelude::*;

// #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
// #[sea_orm(table_name = "post")]
// pub struct Model {
//     #[sea_orm(primary_key)]
//     pub id: i32,
//     pub title: String,
//     pub text: String,
// }

#[shuttle_runtime::main]
async fn main(#[shuttle_secrets::Secrets] secret_store: SecretStore,
        #[shuttle_shared_db::Postgres] db: PgPool) -> ShuttleAxum {

    // let connector = SqlxPostgresConnector::from_sqlx_postgres_pool(db);

    // Get all resources 'the Shuttle way'
    let my_secret = secret_store.get("NAME").unwrap();

    // Use the shared build function
    let router = build_router(my_secret);

    // Let Shuttle do the serving
    Ok(router.into())
}