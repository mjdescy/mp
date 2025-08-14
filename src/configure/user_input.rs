use rustyline::DefaultEditor;

pub fn get_user_input(prompt: &str) -> String {
    let prompt = format!("{}: ", prompt);
    let mut rl = DefaultEditor::new()
        .expect("Failed to create readline editor");
    let readline = rl.readline(&prompt);

    match readline {
        Ok(line) => line.trim().to_string(),
        Err(_) => String::new()
    }
}   
