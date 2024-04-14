use dotenv::dotenv;
use sqlx::mysql::MySqlPool;
use std::env;

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
    let mysql_host = env::var("MYSQL_HOST").unwrap_or("localhost".to_string());
    let database_url =
        format!("mysql://{mysql_user}:{mysql_password}@{mysql_host}:3306/{mysql_database}");
    let pool = MySqlPool::connect(&database_url).await.unwrap();
    // テーブル作成、サンプルレコードの登録
    sqlx::migrate!().run(&pool).await.unwrap();

    // ルーティング設定
    let app = utaha::route::create_router(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
