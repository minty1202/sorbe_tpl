use sorbe_tpl::config;

config! {
    DeepDuplicateConfig => {
        app.server.host: String,
        app.database.url: String,
        app.server.host: u16,
    }
}

fn main() {}
