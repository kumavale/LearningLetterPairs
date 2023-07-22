use axum_server::tls_rustls::RustlsConfig;
use dotenv::dotenv;
use sqlx::mysql::MySqlPool;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // .envファイルの読み込み
    dotenv().ok();
    // ログ出力情報の初期化
    tracing_subscriber::fmt().init();

    // データベースへ接続
    let mysql_user = env::var("MYSQL_USER").unwrap();
    let mysql_password = env::var("MYSQL_PASSWORD").unwrap();
    let mysql_database = env::var("MYSQL_DATABASE").unwrap();
    let database_url =
        format!("mysql://{mysql_user}:{mysql_password}@localhost:3306/{mysql_database}");
    let pool = MySqlPool::connect(&database_url).await.unwrap();
    // テーブル作成、サンプルレコードの登録
    sqlx::migrate!().run(&pool).await.unwrap();

    // ルーティング設定
    let app = backend::route::create_router(pool);

    // https用の設定
    #[allow(unused)]
    let config = RustlsConfig::from_pem_file(
        "../self-signed-certs/cert.pem",
        "../self-signed-certs/key.pem",
    )
    .await
    .unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        //axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
