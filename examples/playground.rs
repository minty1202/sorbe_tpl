use sorbe_tpl::{from_reader, from_reader_with_schema, from_str, from_str_with_schema};
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== sorbe_tpl サンプル ===\n");

    // 1. 基本的な設定ファイルのパース
    basic_parsing_example()?;

    // 2. スキーマ検証付きパース
    schema_validation_example()?;

    // 3. ファイルからの読み込み
    from_reader_example()?;

    // 4. ファイルからの読み込みとスキーマ検証
    file_reading_validation_example()?;

    println!("全ての例が正常に実行されました！");
    Ok(())
}

fn basic_parsing_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("1. 基本的なパース:");

    let config = r#"
        app.name = MyApplication
        app.version = '1.0.0'
        server.host = localhost
        server.port = 8080
        server.ssl_enabled = true
        database.host = 'db.example.com'
        database.port = 5432
        database.max_connections = 100
    "#;

    let result = from_str(config)?;
    println!("パース結果: {}", result);
    println!();

    Ok(())
}

fn schema_validation_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("2. スキーマ検証付きパース:");

    let config = r#"
        app.name = MyApplication
        app.debug = true
        server.port = 8080
        database.max_connections = 100
    "#;

    let schema = r#"
        app.name: string
        app.debug: bool
        server.port: integer
        database.max_connections: unsigned_integer
    "#;

    let result = from_str_with_schema(config, schema)?;
    println!("検証済み結果: {}", result);
    println!();

    Ok(())
}

fn from_reader_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("3. ファイルからの読み込み:");

    let config_file = File::open("examples/config.conf")?;

    let result = from_reader(config_file)?;
    println!("ファイルから読み込み結果: {}", result);
    println!();

    Ok(())
}

fn file_reading_validation_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("4. ファイルからの読み込みとスキーマ検証:");

    let config_file = File::open("examples/config.conf")?;
    let schema_file = File::open("examples/schema.conf")?;

    let result = from_reader_with_schema(config_file, schema_file)?;
    println!("ファイルから読み込み結果: {}", result);
    println!();

    Ok(())
}
