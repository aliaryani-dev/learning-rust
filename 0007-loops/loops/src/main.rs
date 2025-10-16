fn main() {
    // loop loop 
    //loop {
    //    println!("again!");
    // }
    // a while loop can have labels:
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

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

    // for loop in a range
    for i in (1..4).rev() {
        println!("{}",i);
    }
}
