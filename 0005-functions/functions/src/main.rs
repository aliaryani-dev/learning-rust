fn main() {
    another_function();
    yet_another_function(58);
    let x = five();
    println!("x: {}", x);
}

fn another_function() {
    println!("Another function");
}

fn yet_another_function(x:i32) {
    println!("Result: {}", x+2);
}

fn five() -> i32{
    5 // as long as there is no semicolon at the end, this is an expression.
}
