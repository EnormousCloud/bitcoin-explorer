pub mod api;
pub mod args;
pub mod dist;
pub mod rpc;
pub mod telemetry;
pub mod types;

#[derive(Clone)]
pub struct State {
    pub pool: sqlx::Pool<sqlx::postgres::Postgres>,
    pub static_dir: String,
    pub rpc_client: rpc::Client,
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
            pool,
            static_dir: src.static_dir.clone(),
            rpc_client: rpc::Client::new(&src.rpc_addr, &src.rpc_username, &src.rpc_password),
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
    app.at("/api/address/:address").get(api::address);
    app.at("/api/tx/:tx").get(api::transaction);
    app.at("/api/blocks/:block").get(api::block);
    app.at("/api/blocks").get(api::blocks);
    app.at("/api/search").post(api::search);
    // app.at("/api/chainstate").post(api::chainstate);
    app.with(dist::Middleware {});
    app.at("/").get(api::home);
    // app.at("/").get(api::home);
    app.listen(args.listen.as_str()).await?;
    Ok(())
}
