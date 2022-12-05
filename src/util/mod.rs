use std::time::Instant;

pub fn with_timing<F, T>(f: F) -> T
where
    F: Fn() -> T,
{
    let now = Instant::now();

    let result = f();

    let elapsed = now.elapsed();
    println!("time: {:.2?}", elapsed);

    result
}
