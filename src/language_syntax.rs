fn print_sum(a:int, b:int){
    println!("Sum: {}",a+b)
}

fn incr(a:int) -> int {
    a+1
}

fn early_return(a:int) -> int{
    if a>5 {return a;}
    a+1
}

fn main (){
    let x = true;
    let y = if x {5i} else {10i};
    println!("y: {}", y);
    print_sum(3,5);
    println!("5+1: {}",incr(5));
    println!("{}",early_return(6));

    let tuple: (int,&str) = (1,"Hello");
    println!("{}",tuple);
    let (a,b) = tuple;
    println!("Tuple values: {}, {}",a,b);
    println!("Equality check: {}",tuple==(1,"Hello"));
    struct Point {
        x:int,
        y:int,
    }
    let p1 = Point{x:1i,y:1i};
    let p2 = Point{y:2i,x:1i};
    let p3 = Point{x:1i,y:1i};
    println!("x:{} y:{}",p2.x, p2.y);
    struct TupleStruct (int,int,int);
    let x = TupleStruct(1,5,6);
    println!("{} {} {}",x.0,x.1,x.2);
    enum Vehicle{
        Car,
        Tractor,
        Harvester,
        ATV
    }
}
