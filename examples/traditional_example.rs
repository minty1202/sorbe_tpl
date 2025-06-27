use serde::Deserialize;
use sorbe_tpl::{from_reader, from_str};
use std::fs::File;

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct AppConfig {
    app: App,
    server: Server,
    database: Database,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct App {
    name: String,
    version: String,
    debug: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Server {
    host: String,
    port: u16,
    ssl_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Database {
    host: String,
    port: u16,
    max_connections: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== sorbe_tpl マクロ + serde サンプル ===\n");

    manual_struct_from_str_example()?;

    manual_struct_from_reader_example()?;

    println!("全ての例が正常に実行されました！");
    Ok(())
}

fn manual_struct_from_str_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("1. 文字列からの serde を使ったパース:");

    let config_str = r#"
        app.name = WebService
        app.version = '2.1.0'
        app.debug = false
        server.host = '0.0.0.0'
        server.port = 3000
        server.ssl_enabled = true
        database.host = 'postgres.example.com'
        database.port = 5432
        database.max_connections = 100
    "#;

    let config: AppConfig = from_str(config_str)?;

    println!(
        "App: {} v{} (Debug: {})",
        config.app.name, config.app.version, config.app.debug
    );
    println!(
        "Server: {}:{} (SSL: {})",
        config.server.host, config.server.port, config.server.ssl_enabled
    );
    println!(
        "Database: {}:{} (max: {})",
        config.database.host, config.database.port, config.database.max_connections
    );

    println!();
    Ok(())
}

fn manual_struct_from_reader_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("2. ファイルからの serde を使ったパース:");

    let file = File::open("examples/app_config.conf")?;
    let config: AppConfig = from_reader(file)?;

    println!("ファイルから読み込んだ設定:");
    println!("App: {} v{}", config.app.name, config.app.version);
    println!(
        "Server: {}:{} (SSL: {})",
        config.server.host, config.server.port, config.server.ssl_enabled
    );
    println!(
        "Database: {}:{} (max: {})",
        config.database.host, config.database.port, config.database.max_connections
    );

    println!();
    Ok(())
}
