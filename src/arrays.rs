fn main(){
    let list = [1i,2i,3i];
    let list2 = [0i,..20];
    println!("{}",list);
    println!("{}",list2);
    println!("2nd element of list: {}",list[1]);
    let mut vector = vec![1i,2,3,4,5];
    vector.push(6);
    println!("Vector: {}",vector);
    for a in vector.iter(){
        println!("a: {}",a);
    }
    let slice = vector.slice(1,4);
    println!("Slice (1,4): {}",slice);
}
