pub mod args;
// pub mod api;
// pub mod db;
pub mod dist;
pub mod telemetry;

#[derive(Clone)]
pub struct State {
    pub db_pool: sqlx::Pool<sqlx::postgres::Postgres>,
    pub static_dir: String,
}
impl State {
    pub async fn from_args(src: &args::Args) -> State {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(src.database_conn)
            .connect_timeout(std::time::Duration::from_secs(3))
            .connect(src.database_url.as_str())
            .await
            .unwrap();
        if let Err(e) = pool.acquire().await {
            panic!("Database connection failure {} url={}", e, src.database_url);
        };
        Self {
            db_pool: pool,
            static_dir: src.static_dir.clone(),
        }
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new("sqlx=warn,info"))
        .init();
    let args = match args::parse() {
        Ok(x) => x,
        Err(e) => panic!("Args parsing error: {}", e),
    };

    let mut app = tide::with_state(State::from_args(&args).await);
    app.with(telemetry::TraceMiddleware::new());
    app.with(dist::Middleware {});
    // app.at("/").get(homepage::get);
    app.listen(args.listen.as_str()).await?;
    Ok(())
}
