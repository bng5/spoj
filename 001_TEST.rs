use std::io;

fn main() {
    let mut input = String::new();
    loop {
        
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        if input == "42" {
            break;
        }
        println!("{}", input);
        input.clear();
    }
}
