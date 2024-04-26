pub mod users {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct FirebaseUser {
        pub name: String,
    }
}
