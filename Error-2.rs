fn main() {
    let text = "Hello, World!";
    
    let character_option = text.chars().nth(15);
    
    // using match for Option type
    let character = match character_option {
        None => "empty".to_string(),
        Some(c) => c.to_string()
    };
    
    println!("Character at index 15 is {}", character);
}
