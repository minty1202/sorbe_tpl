use sorbe_tpl::config;

config! {
    InvalidFieldConfig => {
        123invalid: String,
        -invalid: u32,
        invalid.: bool,
    }
}

fn main() {}
