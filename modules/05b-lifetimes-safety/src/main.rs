fn main() {
    let x = 10;
    let _max: &i32;
    {
        let y = 20;
        _max = find_max(&x, &y);
    }
    // println!("Max value is {}", *_max);
}

fn find_max<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y { x } else { y }
}
