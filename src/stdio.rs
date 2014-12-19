use std::io;

fn main(){
    println!("Enter text: ");
    let input = io::stdin().read_line().ok().expect("Failure");
    println!("You entered: {}", input);
}
