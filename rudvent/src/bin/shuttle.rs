use shuttle_axum::ShuttleAxum;
use shuttle_secrets::SecretStore;
use rudvent::build_router;


#[shuttle_runtime::main]
async fn axum(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> ShuttleAxum {
    // Get all resources 'the Shuttle way'
    let my_secret = secret_store.get("NAME").unwrap();

    // Use the shared build function
    let router = build_router(my_secret);

    // Let Shuttle do the serving
    Ok(router.into())
}