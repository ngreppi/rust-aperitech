fn main() {
    let x = 10;
    let max: &i32;
    {
        let y = 20;
        max = find_max(&x, &y);
    }
    // println!("Max value is {}", *max);
}

fn find_max<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y { x } else { y }
}
