use std::fmt;

use rand::Rng;

fn main() {
    let mut my_data = Data { descr: String::from("My data feed"), feed: vec![0.0; 10] };
    let mut _my_data_2 = &mut my_data;
    enrich_data(&mut my_data);
    // _my_data_2.descr = String::from("My data feed 2");
    println!("{my_data}");
}

fn enrich_data(my_data: &mut Data) {
    let mut rng = rand::thread_rng();
    my_data.feed.clear();
    for _ in 0..10 {
        my_data.feed.push(rng.gen_range(0.0..1.0));
    }
}

struct Data {
    descr: String,
    feed: Vec<f64>,
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Description: {}", self.descr)?;
        writeln!(f, "Feed: [")?;
        for value in &self.feed {
            writeln!(f, "  {:.4},", value)?;
        }
        writeln!(f, "]")
    }
}
