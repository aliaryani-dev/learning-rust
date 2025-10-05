fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    // x = 6; // ERROR
    x = 6;
    println!("The value of x is {}", x);

    const MAX_POINT:u32 = 100_000; // a constant here is like constexpr in C++
    println!("MAX_POINT: {}", MAX_POINT);

    // shadowing
    let y = 12;
    let y = y + 3;
    let y = y * 2;
    println!("The value of y: {}", y);
}
