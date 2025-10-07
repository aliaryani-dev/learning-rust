fn main() {
    let _a:u8 = 255; // unsigned int
    let _b = 3.0; // f64
    let _c = false; // bool
    let _d = 'D'; // char 

    // compound types
    // tuple
    let tup: (i32,f64,u8) = (12,3.4,5);
    let (x, y, z) = tup;
    println!("x: {} | y: {}, z: {}", x,y,z);
}
