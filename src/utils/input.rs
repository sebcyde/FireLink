pub mod input {
    use dialoguer::MultiSelect;

    pub fn create_input_prompt(prompt: &str, mut options: Vec<&str>) -> String {
        println!("\n{}", prompt);
        println!("SPACEBAR to select, and then ENTER to submit");
        options.sort_by(|a, b| a.cmp(b));
        let selection: Vec<usize> = MultiSelect::new().items(&options).interact().unwrap();
        let selected_option: &str = options[*selection.first().unwrap()];
        println!("Selected: {}\n", selected_option);
        return String::from(selected_option);
    }
}
