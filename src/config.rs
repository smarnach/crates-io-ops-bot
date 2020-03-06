pub mod config {
    use std::sync::Arc;
    use serenity::prelude::TypeMapKey;

    #[derive(Debug)]
    pub struct Config {
        pub discord_token: String,
        pub authorized_users: String,
        pub heroku_api_key: String,
    }

    impl Config {
        pub fn new(
            discord_token: String,
            authorized_users: String,
            heroku_api_key: String,
        ) -> Config {
            Config {
                discord_token,
                authorized_users,
                heroku_api_key,
            }
        }
    }

    impl TypeMapKey for Config {
        type Value = Arc<Config>;
    }
}
