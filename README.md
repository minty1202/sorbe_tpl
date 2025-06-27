# sorbe_tpl

Rust アプリケーション向けの設定ファイル解析ライブラリです。ドット記法による階層構造と Serde 統合により、型安全な設定管理を実現します。

## 特徴

- **ドット記法**: `app.name = value` 形式による設定記述
- **型安全性**: Serde との統合により構造体への自動デシリアライゼーション
- **スキーマ検証**: 設定値の型チェックとバリデーション
- **柔軟な読み込み**: 文字列・ファイル・リーダーからの解析をサポート
- **マクロ**: `config!` マクロによる設定構造体の簡単定義

## インストール

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
sorbe_tpl = { git = "https://github.com/minty1202/sorbe_tpl.git", branch = "main" }
```

## 基本的な使用方法

### 1. 基本的な解析

```rust
use sorbe_tpl::{Value, from_str};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = r#"
        app.name = MyApplication
        app.version = '1.0.0'
        server.host = localhost
        server.port = 8080
        server.ssl_enabled = true
        database.max_connections = 100
    "#;

    let result: Value = from_str(config)?;
    println!("設定: {}", result);
    Ok(())
}
```

### 2. Serde を使用した構造体への解析

```rust
use serde::Deserialize;
use sorbe_tpl::from_str;

#[derive(Debug, Deserialize)]
struct AppConfig {
    app: App,
    server: Server,
    database: Database,
}

#[derive(Debug, Deserialize)]
struct App {
    name: String,
    version: String,
}

#[derive(Debug, Deserialize)]
struct Server {
    host: String,
    port: u16,
    ssl_enabled: bool,
}

#[derive(Debug, Deserialize)]
struct Database {
    max_connections: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_str = r#"
        app.name = WebService
        app.version = '2.1.0'
        server.host = '0.0.0.0'
        server.port = 3000
        server.ssl_enabled = true
        database.max_connections = 100
    "#;

    let config: AppConfig = from_str(config_str)?;
    println!("アプリ名: {}", config.app.name);
    println!("サーバー: {}:{}", config.server.host, config.server.port);
    Ok(())
}
```

### 3. config! マクロを使用した定義

`config!` マクロを使用すると、手動で構造体を定義する必要がありません。マクロが自動的に階層構造に対応した構造体を生成し、`Debug` `Clone` `PartialEq` `Serialize` `Deserialize` トレイトを付与します。

```rust
use sorbe_tpl::{config, from_str};

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

// 上記のマクロは以下のような構造体を自動生成します：
// #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
// struct AppConfig {
//     app: App,
//     server: Server,
//     database: Database,
// }
// 
// #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
// struct App {
//     name: String,
//     version: String,
//     debug: bool,
// }
// ... など

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let config: AppConfig = from_str(config_str)?;
    println!("アプリ: {} v{}", config.app.name, config.app.version);
    Ok(())
}
```

### 4. ファイルからの読み込み

```rust
use sorbe_tpl::from_reader;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("config.conf")?;
    let config: AppConfig = from_reader(file)?;
    println!("設定を読み込みました: {:?}", config);
    Ok(())
}
```

### 5. スキーマ検証付き解析

```rust
use sorbe_tpl::from_str_with_schema;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    println!("検証済み設定: {}", result);
    Ok(())
}
```

## 設定ファイル形式

### 基本構文

```conf
# コメント
app.name = MyApplication
app.version = '1.0.0'
server.host = localhost
server.port = 8080
server.ssl_enabled = true
database.max_connections = 100
```

### サポートされるデータ型

- **文字列**: `name = value` または `name = 'quoted value'`
- **整数**: `port = 8080`
- **浮動小数点数**: `timeout = 30.5`
- **真偽値**: `enabled = true` / `enabled = false`

### 階層構造

ドット記法により階層構造を表現：

```conf
app.name = MyApp
app.server.host = localhost
app.server.port = 8080
app.database.host = db.example.com
app.database.credentials.username = admin
```

## API リファレンス

### 主要関数

- `from_str<T>(input: &str) -> Result<T, Error>` - 文字列から解析
- `from_reader<R, T>(reader: R) -> Result<T, Error>` - Readerから解析
- `from_str_with_schema(config: &str, schema: &str) -> Result<Value, Error>` - スキーマ検証付き解析
- `from_reader_with_schema<R1, R2>(config: R1, schema: R2) -> Result<Value, Error>` - ファイルからスキーマ検証付き解析

### マクロ

- `config! { StructName => { field.path: Type, ... } }` - 設定構造体の定義

## スキーマ型

スキーマ検証で使用可能な型：

- `string` - 文字列
- `integer` - 符号付き整数
- `unsigned_integer` - 符号なし整数
- `float` - 浮動小数点数
- `bool` - 真偽値

## エラーハンドリング

```rust
use sorbe_tpl::{from_str, Error};

match from_str::<AppConfig>(config_str) {
    Ok(config) => println!("設定読み込み成功: {:?}", config),
    Err(Error::Lexer(lexer_err)) => eprintln!("字句解析エラー: {}", lexer_err),
    Err(Error::Parse(parse_err)) => eprintln!("構文解析エラー: {}", parse_err),
    Err(Error::UnknownKey { key }) => eprintln!("未知のキー: {}", key),
    Err(Error::MissingKey { key }) => eprintln!("必須キーが不足: {}", key),
    Err(Error::TypeMismatch { expected, found }) => {
        eprintln!("型不一致: {} を期待しましたが {} が見つかりました", expected, found)
    }
    Err(Error::Serde(msg)) => eprintln!("Serdeエラー: {}", msg),
    Err(Error::Io(io_err)) => eprintln!("IOエラー: {}", io_err),
}
```

## ライセンス

MIT License
