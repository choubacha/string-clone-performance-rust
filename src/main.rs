use std::time;

fn time<F>(name: &str, iterations: usize, mut f: F) where F: FnMut() {
    let start = time::Instant::now();
    for _ in 0..iterations { f(); }
    let total = start.elapsed();
    let secs = total.as_secs() as f64 + (total.subsec_nanos() as f64 / 1_000_000_000.0);
    println!("{:>20}: {:>15.2} i/sec", name, iterations as f64 / secs);
}

fn main() {
    let s = "r".repeat(100).to_string();
    time("100 rs", 100_000_000, || { s.clone(); });
    let s = "r".repeat(1000).to_string();
    time("1000 rs", 100_000_000, || { s.clone(); });
    let s = "r".repeat(10_000).to_string();
    time("10,000 rs", 100_000_000, || { s.clone(); });
    let s = "r".repeat(100_000).to_string();
    time("100,000 rs", 10_000_000, || { s.clone(); });
}
