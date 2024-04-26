pub mod config {
    use std::collections::HashMap;
    use std::fs::{read_to_string, File};
    use std::io::Write;
    use std::path::PathBuf;

    use firebase_rs::Firebase;
    use log::{error, info, warn};

    use crate::firebase::auth::auth::{get_users, set_user};
    use crate::types::types::FireLinkConfig;
    use crate::user_types::users::FirebaseUser;
    use crate::utils::input::input::{create_input_prompt, get_user_input};

    fn get_firelink_config_dir_path() -> PathBuf {
        let mut config_path: PathBuf = dirs::config_dir().unwrap();
        config_path.push("FireLink");
        return config_path;
    }

    fn get_firelink_config_file_path() -> PathBuf {
        let mut config_path: PathBuf = get_firelink_config_dir_path();
        config_path.push("firelink_config.json");
        return config_path;
    }

    fn get_default_user_config() -> FireLinkConfig {
        let empty_user_config: FireLinkConfig = FireLinkConfig::default();
        warn!("Empty user config detected.",);
        return empty_user_config;
    }

    pub fn get_user_config() -> FireLinkConfig {
        let config_value: &str = &read_to_string(get_firelink_config_file_path()).unwrap();
        let config: FireLinkConfig = serde_json::from_str(config_value).unwrap();
        return config;
    }

    fn create_config_dir_and_files() {
        let config_dir_path: PathBuf = get_firelink_config_dir_path();
        let dir_res: Result<(), std::io::Error> = std::fs::create_dir_all(&config_dir_path);
        if dir_res.is_err() {
            error!("Error creating config directory.");
        }

        create_default_config_files();
    }

    fn create_default_config_files() {
        let default_config: FireLinkConfig = get_default_user_config();
        let config_file_path: PathBuf = get_firelink_config_file_path();
        let config_dir_path: PathBuf = get_firelink_config_dir_path();

        _ = std::fs::create_dir_all(&config_dir_path);

        let mut config_file: File = File::create(&config_file_path).unwrap();
        let json_data: String = serde_json::to_string(&default_config).unwrap();
        _ = config_file.write_all(json_data.as_bytes());

        info!("Config created succesfully.");
    }

    pub fn update_config(new_config: FireLinkConfig) {
        let config_file_path: PathBuf = get_firelink_config_file_path();
        let config_dir_path: PathBuf = get_firelink_config_dir_path();

        _ = std::fs::create_dir_all(&config_dir_path);

        let mut config_file: File = File::create(&config_file_path).unwrap();
        let json_data: String = serde_json::to_string(&new_config).unwrap();
        _ = config_file.write_all(json_data.as_bytes());

        info!("Config updated succesfully.");
    }

    pub async fn check_config() -> FireLinkConfig {
        let firelink_config: FireLinkConfig = get_config_data();
        println!("FireLinkConfig: {:?}", &firelink_config);

        if firelink_config.username.is_empty() {
            warn!("Empty username found in config.");

            loop {
                let new_username: String = get_user_input("Please enter a new username:");
                let username_prompt: String = format!("You entered {}. Is this ok?", &new_username);
                let options: Vec<&str> = vec!["No", "Yes"];

                let response: String = create_input_prompt(&username_prompt, options);
                println!("RESPONSE: {}", response);

                if response.eq_ignore_ascii_case("yes") {
                    // Check if it exists in the database for anyone else
                    println!("Checking if username is available");

                    let firebase: Firebase = Firebase::new(
                        "https://firelink-50aeb-default-rtdb.europe-west1.firebasedatabase.app/",
                    )
                    .unwrap();

                    let users: HashMap<String, crate::user_types::users::FirebaseUser> =
                        get_users(&firebase).await;

                    let mut acceptable_username: bool = true;

                    for entry in users.iter() {
                        let user: &FirebaseUser = entry.1;
                        if user.name.eq_ignore_ascii_case(&new_username) {
                            println!(
                                "Username is already taken. Please enter a different username."
                            );

                            acceptable_username = false;
                        };
                    }

                    if acceptable_username {
                        // Create FireBase User and update in local config
                        let new_user: FirebaseUser = FirebaseUser {
                            name: new_username.clone(),
                        };

                        let new_config: FireLinkConfig = FireLinkConfig {
                            username: new_username.clone(),
                        };

                        set_user(&firebase, &new_user).await;
                        update_config(new_config);
                        break;
                    }
                }
            }
        }

        return get_config_data();
    }

    pub fn _check_config() -> bool {
        info!("Checking FireLink config.");

        let config_file_path: PathBuf = get_firelink_config_file_path();
        let config_dir_path: PathBuf = get_firelink_config_dir_path();

        if std::path::Path::exists(&config_dir_path) {
            if std::path::Path::exists(&config_file_path) {
                // Dir and config Exists

                let config: FireLinkConfig = get_user_config();

                if config.any_field_empty() {
                    warn!("Config not set. Using default.");
                    return false;
                } else {
                    return true;
                }
            } else {
                create_default_config_files();
                return false;
            }
        } else {
            println!("Creating config. Please wait...");
            create_config_dir_and_files();
            return false;
        }
    }

    pub fn get_config_data() -> FireLinkConfig {
        let config_file_path: PathBuf = get_firelink_config_file_path();
        if !config_file_path.exists() {
            create_default_config_files();
        }
        let config_value: &str = &read_to_string(config_file_path).unwrap();
        let config: FireLinkConfig = serde_json::from_str(config_value).unwrap();
        return config;
    }
}
