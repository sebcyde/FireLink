pub mod input {
    use dialoguer::MultiSelect;
    use std::io::Write;

    pub fn create_input_prompt(prompt: &str, mut options: Vec<&str>) -> String {
        println!("\n{}", prompt);
        println!("SPACEBAR to select, and then ENTER to submit");
        options.sort_by(|a, b| a.cmp(b));
        let selection: Vec<usize> = MultiSelect::new().items(&options).interact().unwrap();
        let selected_option: &str = options[*selection.first().unwrap()];
        println!("Selected: {}\n", selected_option);
        return String::from(selected_option);
    }

    pub fn get_user_input(prompt: &str) -> String {
        println!("{}", prompt);
        std::io::stdout().flush().expect("Failed to flush stdout");

        let mut input: String = String::new(); // Create a new String to store user input
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line"); // Read user input from stdin

        return input.trim().to_string();
    }
}
