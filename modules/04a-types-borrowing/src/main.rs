fn main() {
    let my_code = String::from("OK");
    if validate(&my_code) {
        println!("Code {} is valid!", my_code);
    } else {
        println!("Code {} is NOT valid!", my_code);
    }
}

fn validate(code: &String) -> bool {
    match code.as_str() {
        "OK" => true,
        "KO" => true,
        _ => false,
    }
}
