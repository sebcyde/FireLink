pub mod auth {

    use std::collections::HashMap;

    use firebase_rs::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct User {
        pub name: String,
        pub age: u32,
        pub email: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Response {
        pub name: String,
    }

    pub async fn set_user(firebase_client: &Firebase, user: &User) -> Response {
        let firebase = firebase_client.at("users");
        let _users = firebase.set::<User>(&user).await;
        return string_to_response(&_users.unwrap().data);
    }

    pub async fn get_users(firebase_client: &Firebase) -> HashMap<String, User> {
        let firebase = firebase_client.at("users");
        let users = firebase.get::<HashMap<String, User>>().await;
        return users.unwrap();
    }

    pub async fn get_user(firebase_client: &Firebase, id: &String) -> User {
        let firebase = firebase_client.at("users").at(&id);
        let user = firebase.get::<User>().await;
        return user.unwrap();
    }

    pub async fn update_user(firebase_client: &Firebase, id: &String, user: &User) -> User {
        let firebase = firebase_client.at("users").at(&id);
        let _user = firebase.update::<User>(&user).await;
        return string_to_user(&_user.unwrap().data);

        // Example
        // user.email = "updated.mail@gmail.com".to_string();
        // let updated_user = update_user(&firebase, &response.name, &user).await;
        // println!("{:?}", updated_user);
    }

    pub async fn delete_user(firebase_client: &Firebase, id: &String) {
        let firebase = firebase_client.at("users").at(&id);
        let _result = firebase.delete().await;

        // Example
        // delete_user(&firebase, &response.name).await;
        // println!("User deleted!");
    }

    // convert a string to a response
    fn string_to_response(s: &str) -> Response {
        serde_json::from_str(s).unwrap()
    }

    //convert a string to a user
    fn string_to_user(s: &str) -> User {
        serde_json::from_str(s).unwrap()
    }
}
