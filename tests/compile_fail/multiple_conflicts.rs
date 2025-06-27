use sorbe_tpl::config;

config! {
    MultipleConflictConfig => {
        app: bool,
        app.name: String,
        app.version: String,
        database: u32,
        database.host: String,
    }
}

fn main() {}
