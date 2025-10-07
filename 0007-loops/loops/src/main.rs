fn main() {
    // loop loop 
    //loop {
    //    println!("again!");
    // }
    // while loop
    let mut number = 5;
    while number != 0 {
        println!("again!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
    
    // for loop for iteration
    let a = [10,20,30,40,50];
    for element in a.iter() {
        println!("the value is {}", element);
    }
}
