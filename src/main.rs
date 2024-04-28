mod config;
mod firebase;
mod types;
mod user_types;
mod utils;

use config::fire_link_config::config::check_config;
use firebase::storage::uploads::get_storage_buckets;
use firebase_rs::Firebase;
use log::warn;
use types::types::FireLinkConfig;
use utils::input::input::{create_input_prompt, get_user_input};
use utils::logs::logs::setup_logger;

#[tokio::main]
async fn main() {
    println!("Loading...");

    _ = setup_logger();

    let firelink_config: FireLinkConfig = check_config().await;
    println!("FireLinkConfig: {:?}", &firelink_config);

    get_storage_buckets().await;

    // let user = User {
    //     name: "JTSC".to_string(),
    //     age: 26,
    //     email: "JTSC@test.com".to_string(),
    // };

    // let firebase: Firebase =
    //     Firebase::new("https://firelink-50aeb-default-rtdb.europe-west1.firebasedatabase.app/")
    //         .unwrap();

    // let response: firebase::auth::auth::Response = set_user(&firebase, &user).await;

    // let user: User = get_user(&firebase, &response.name).await;
    // println!("{:?}", user);

    // let args: Vec<String> = std::env::args().collect();

    // First argument is always program name
    // if let Some(program_name) = args.get(0) {
    //     println!("Program name: {}", program_name);
    // }

    // // Check if a file path was provided as an argument
    // if let Some(file_path) = args.get(1) {
    //     match read_file(file_path) {
    //         Ok(file_contents) => {
    //             // let user = authenticate_user();
    //             // upload_file_to_storage();
    //         }
    //         Err(err) => eprintln!("Error reading file: {}", err),
    //     }
    // } else {
    //     eprintln!("Usage: {} <file_path>", args[0]);
    // }
}
