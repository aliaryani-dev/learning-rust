fn main() {
    another_function();
    yet_another_function(58);
}

fn another_function() {
    println!("Another function");
}

fn yet_another_function(x:i32) {
    println!("Result: {}", x+2);
}
