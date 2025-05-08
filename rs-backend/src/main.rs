mod app;
mod middleware;
mod models;
mod routes;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();

    let app = app::build_app().await?;
    println!("Server running at http://localhost:4000");
    app::run_server(app, "[::]:4000").await.unwrap();

    Ok(())
}
