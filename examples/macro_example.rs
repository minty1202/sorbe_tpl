use sorbe_tpl::config;
use std::fs::File;

config! {
    AppConfig => {
        app.name: String,
        app.version: String,
        app.debug: bool,
        server.host: String,
        server.port: u16,
        server.ssl_enabled: bool,
        database.host: String,
        database.port: u16,
        database.max_connections: u32,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== sorbe_tpl マクロ + serde サンプル ===\n");

    string_parsing_example()?;

    file_parsing_example()?;

    println!("全ての例が正常に実行されました！");
    Ok(())
}

fn string_parsing_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("1. 文字列からのパース:");

    let config_str = r#"
        app.name = MyApplication
        app.version = '2.0.0'
        app.debug = true
        server.host = localhost
        server.port = 8080
        server.ssl_enabled = false
        database.host = 'db.example.com'
        database.port = 5432
        database.max_connections = 50
    "#;

    let config: AppConfig = sorbe_tpl::from_str(config_str)?;

    println!("App: {}", config.app.name);
    println!("Version: {}", config.app.version);
    println!("Debug: {}", config.app.debug);
    println!("Server: {}:{}", config.server.host, config.server.port);
    println!("SSL: {}", config.server.ssl_enabled);
    println!("DB: {}:{}", config.database.host, config.database.port);
    println!("Max Connections: {}", config.database.max_connections);
    println!();

    Ok(())
}

fn file_parsing_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("2. ファイルからの serde を使ったパース:");

    let file = File::open("examples/app_config.conf")?;
    let config: AppConfig = sorbe_tpl::from_reader(file)?;

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
