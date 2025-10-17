fn main() {
    let number = 4;
    if number < 5 {
        println!("smaller!");
    } else {
        println!("bigger!");
    }

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
}
