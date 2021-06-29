pub mod api;
pub mod args;
// pub mod db;
pub mod dist;
pub mod telemetry;

pub extern crate bitcoincore_rpc;
pub extern crate bitcoincore_rpc_json;

#[derive(Clone)]
pub struct State {
    pub pool: sqlx::Pool<sqlx::postgres::Postgres>,
    pub static_dir: String,
    pub rpc_addr: String,
    pub rpc_auth: bitcoincore_rpc::Auth,
}

impl State {
    pub async fn from_args(src: &args::Args) -> State {
        let rpc_auth = if src.rpc_username.len() > 0 {
            bitcoincore_rpc::Auth::UserPass(src.rpc_username.clone(), src.rpc_password.clone())
        } else {
            bitcoincore_rpc::Auth::None
        };
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
            rpc_addr: src.rpc_addr.clone(),
            rpc_auth,
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
    app.at("/").get(api::home);
    app.listen(args.listen.as_str()).await?;
    Ok(())
}
