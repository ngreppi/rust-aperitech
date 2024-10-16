use std::fmt;

fn main() {
    let my_type: MyType<u32> = MyType::new(String::from("My u32 param"), 10);
    println!("Value of my_type: {my_type}");
    // my_type.value = 15;
    println!("Value of my_type: {my_type}");
}

struct MyType<T> {
    value: T,
    descr: String,
}

impl<T> MyType<T> {
    fn new(descr: String, value: T) -> Self {
        MyType { value, descr }
    }
}

impl<T> fmt::Display for MyType<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Param descr: {}, value: {}", self.descr, self.value)
    }
}
