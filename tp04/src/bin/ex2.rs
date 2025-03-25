use std::io::{self, Write};

fn main() {
   
    let mut boxed_string: Box<String> = Box::new(String::new());

    print!("Veuillez entrer un texte: ");
    io::stdout().flush().unwrap(); 
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input = input.trim().to_string();

    boxed_string.push_str(&input);
    
    println!("Hallo World: '{}'", boxed_string);
} fm main




