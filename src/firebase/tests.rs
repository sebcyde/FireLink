#[cfg(test)]
mod tests {
    use firebase_rs::Firebase;

    use crate::{
        firebase::auth::auth::{delete_user, get_user, get_users, set_user, update_user, Response},
        user_types::users::FirebaseUser,
    };

    #[tokio::test]
    async fn create_firebase_user() {
        let firebase: Firebase =
            Firebase::new("https://firelink-50aeb-default-rtdb.europe-west1.firebasedatabase.app/")
                .unwrap();

        let test_user: FirebaseUser = FirebaseUser {
            name: String::from("TEST--USER--ACCOUNT"),
        };

        // Testing user creation
        let test_user_res: Response = set_user(&firebase, &test_user).await;
        let user: FirebaseUser = get_user(&firebase, &test_user_res.name).await;
        assert_eq!(user.name, "TEST--USER--ACCOUNT");

        // Test user deletion
        delete_user(&firebase, &test_user_res.name).await;
        let all_users = get_users(&firebase).await;

        for (_, single_user) in all_users.iter() {
            assert_ne!(single_user.name, "TEST--USER--ACCOUNT")
        }
    }

    #[tokio::test]
    async fn update_firebase_user() {
        let firebase: Firebase =
            Firebase::new("https://firelink-50aeb-default-rtdb.europe-west1.firebasedatabase.app/")
                .unwrap();

        // Creating initial user
        let test_user: FirebaseUser = FirebaseUser {
            name: String::from("UPDATING--TEST--USER--ACCOUNT"),
        };

        let test_user_res: Response = set_user(&firebase, &test_user).await;
        let user: FirebaseUser = get_user(&firebase, &test_user_res.name).await;
        assert_eq!(user.name, "UPDATING--TEST--USER--ACCOUNT");

        // Testing updated user details
        let new_test_user: FirebaseUser = FirebaseUser {
            name: String::from("UPDATED--TEST--USER--ACCOUNT"),
        };

        update_user(&firebase, &test_user_res.name, &new_test_user).await;
        let updated_user: FirebaseUser = get_user(&firebase, &test_user_res.name).await;
        assert_eq!(updated_user.name, "UPDATED--TEST--USER--ACCOUNT");

        delete_user(&firebase, &test_user_res.name).await;
        let all_users = get_users(&firebase).await;
        for (_, single_user) in all_users.iter() {
            assert_ne!(single_user.name, "TEST--USER--ACCOUNT")
        }
    }
}
