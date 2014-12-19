fn match_int(a:int){
    match a{
        1=>println!("One"),
        2=>println!("Two"),
        3=>println!("Three"),
        4=>println!("Four"),
        5=>println!("Five"),
        _=>println!("Other")
    }
}

fn main(){
    match_int(5);
}
