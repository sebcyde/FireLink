pub mod types {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct FireLinkConfig {
        pub username: String,
    }

    impl FireLinkConfig {
        pub fn any_field_empty(&self) -> bool {
            self.username.is_empty()
            // || self.misc_location.is_empty()
        }
    }

    impl Default for FireLinkConfig {
        fn default() -> Self {
            FireLinkConfig {
                username: String::new(),
            }
        }
    }
}
