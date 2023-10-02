use std::{time::{Duration, Instant}, fmt::{Display, Write}};

struct Wrapper<T>(T);

impl<T: Display> Display for Wrapper<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

fn main() {
    let (output, elapsed) = measure_time(|| {
        let mut output = String::new();
        for i in 0..100000000 {
            write!(output, "hello {} {}!", "world", i).unwrap();
        }
        output
    });
    println!("no wrapper millis: {}, no wrapper length: {}", elapsed.as_millis(), output.len());

    let (output, elapsed) = measure_time(|| {
        let mut output = String::new();
        for i in 0..100000000 {
            write!(output, "hello {} {}!", Wrapper("world"), i).unwrap();
        }
        output
    });
    println!("with wrapper millis: {}, with wrapper length: {}", elapsed.as_millis(), output.len());
}

fn measure_time<T, F: FnOnce() -> T>(f: F) -> (T, Duration) {
    let start_time = Instant::now();
    let result = f();
    let elapsed_time = start_time.elapsed();

    (result, elapsed_time)
}
