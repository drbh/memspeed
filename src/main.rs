use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::thread;
use std::time::Instant;

const NUM_THREADS: usize = 8;
const BUFFER_SIZE: usize = // 32 MB
    32 * 1_024 * 1_024 / std::mem::size_of::<f64>();
const NUM_ITERATIONS: usize = 1_000;

struct ThreadData {
    buffer: Vec<f64>,
    id: usize,
}

fn measure_bandwidth(data: ThreadData) {
    let mut dummy = 0.0;
    for _ in 0..NUM_ITERATIONS {
        for &value in data.buffer.iter() {
            dummy += value;
        }
    }
    println!("• Thread {} dummy: {}", data.id, dummy);
}

fn main() {
    let mut threads = vec![];
    let mut thread_data = vec![];

    let seed: u64 = 42; // Use a fixed seed for deterministic behavior
    let mut rng = StdRng::seed_from_u64(seed);

    println!("Generating random data...");
    for i in 0..NUM_THREADS {
        let buffer: Vec<f64> = (0..BUFFER_SIZE).map(|_| rng.gen_range(0.0..1.0)).collect();
        let data = ThreadData { buffer, id: i };
        thread_data.push(data);
    }

    let bytes_generated = NUM_THREADS * BUFFER_SIZE * std::mem::size_of::<f64>();

    println!(
        "Generated {:.2} GB of random data.",
        bytes_generated as f64 / 1e9
    );

    let start = Instant::now();

    for data in thread_data {
        let handle = thread::spawn(move || measure_bandwidth(data));
        threads.push(handle);
    }

    for handle in threads {
        handle.join().unwrap();
    }

    let elapsed_time = start.elapsed().as_secs_f64();

    println!("Elapsed time: {} seconds", elapsed_time);

    let bandwidth = (NUM_THREADS * BUFFER_SIZE * NUM_ITERATIONS) as f64
        * std::mem::size_of::<f64>() as f64
        / elapsed_time;
    println!("Memory bandwidth: {} GB/s", bandwidth / 1e9);
}
