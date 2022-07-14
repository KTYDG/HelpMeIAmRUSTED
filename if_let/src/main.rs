fn main() {
    let config_max: Option<u8> = Some(255);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
