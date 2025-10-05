fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    // x = 6; // ERROR
    x = 6;
    println!("The value of x is {}", x);

    const MAX_POINT:u32 = 100_000; // a constant here is like constexpr in C++
    println!("MAX_POINT: {}", MAX_POINT);
}
