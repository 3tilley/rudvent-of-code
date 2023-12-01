use poem::middleware::Cors;
use poem::{listener::TcpListener, EndpointExt, Route, Server, Result};
use poem_openapi::param::Path;
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService};
use poem_openapi::payload::Json;
use rudvent_lib::http::models::AdventSolution;

struct Api;

#[OpenApi]
impl Api {
    /// Hello world
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
        PlainText("Hello World")
    }

    /// Double a number
    #[oai(path = "/double/:number", method = "get")]
    async fn double(&self, Path(number): Path<u64>) -> PlainText<String> {
        PlainText(format!("{} * 2 = {}", number, number * 2))
    }

    /// Add two numbers
    #[oai(path = "/add/:left/:right", method = "get")]
    async fn add(&self, Path(left): Path<u64>, Path(right): Path<u64>) -> PlainText<String> {
        PlainText(format!("{} + {} = {}", left, right, rudvent_lib::add(left, right)))
    }

    /// Get a solution
    #[oai(path = "/solution/:id", method = "get")]
    async fn get_solution(&self, Path(id): Path<u64>) -> PlainText<String> {
        let solution = get_solutions().get(0);
        PlainText(format!("{:?}", solution))
    }

    #[oai(path = "/solutions", method = "get")]
    async fn solutions(&self) -> Result<Json<Vec<AdventSolution>>> {
        let solutions = get_solutions();
        Ok(Json(solutions))
    }

}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("http://localhost:8000");
    let ui = api_service.swagger_ui();
    let app = Route::new()
        .nest("/", api_service)
        .nest("/docs", ui)
        .with(Cors::new());

    Server::new(TcpListener::bind("127.0.0.1:8000"))
        .run(app)
        .await
}
