// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

 
fn main() {
    let data = "Rust is great!".to_string();

    get_char(data.clone());

    let uppercase_data = string_uppercase(&data);
    println!("{}", uppercase_data);
}

fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

fn string_uppercase(data: &String) -> String {
    data.to_uppercase()
}
