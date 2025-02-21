use std::time::{Instant, Duration};
use std::thread;
use std::sync::Arc;

const MATRIX_SIZE: usize = 10;
const NUM_OPERATIONS: usize = 1000;
const NUM_THREADS: usize = 3;
const NUM_MATRICES: usize = 30;

type Matrix = Vec<Vec<f64>>;

fn generate_matrices() -> Vec<Matrix> {
    let mut matrices = Vec::with_capacity(NUM_MATRICES);
    for i in 0..NUM_MATRICES {
        let mut matrix = Vec::with_capacity(MATRIX_SIZE);
        for row in 0..MATRIX_SIZE {
            let mut row_vec = Vec::with_capacity(MATRIX_SIZE);
            for col in 0..MATRIX_SIZE {
                row_vec.push((i * 100 + row * 10 + col) as f64); // Example predefined numbers
            }
            matrix.push(row_vec);
        }
        matrices.push(matrix);
    }
    matrices
}

fn multiply_matrices(a: &Matrix, b: &Matrix) -> Matrix {
    let mut result = vec![vec![0.0; MATRIX_SIZE]; MATRIX_SIZE];
    for i in 0..MATRIX_SIZE {
        for j in 0..MATRIX_SIZE {
            for k in 0..MATRIX_SIZE {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    result
}

fn benchmark_thread(matrices: Arc<Vec<Matrix>>, thread_id: usize) -> Duration {
    let start_time = Instant::now();
    for i in 0..(NUM_OPERATIONS / NUM_THREADS) {
        let matrix_index1 = (i + thread_id * (NUM_OPERATIONS / NUM_THREADS)) % NUM_MATRICES;
        let matrix_index2 = (matrix_index1 + 1) % NUM_MATRICES; // Ensure different matrices
        multiply_matrices(&matrices[matrix_index1], &matrices[matrix_index2]);
    }
    start_time.elapsed()
}

fn main() {
    let matrices = Arc::new(generate_matrices()); // Arc for sharing matrices across threads
    let mut threads = Vec::new();
    let total_start_time = Instant::now();

    for thread_id in 0..NUM_THREADS {
        let matrices_clone = matrices.clone(); // Clone Arc for each thread
        threads.push(thread::spawn(move || benchmark_thread(matrices_clone, thread_id)));
    }

    let mut total_duration = Duration::new(0, 0);
    for thread_join_handle in threads {
        total_duration += thread_join_handle.join().unwrap();
    }

    let total_elapsed_seconds = total_start_time.elapsed().as_secs_f64();

    println!("Rust Benchmark Results:");
    println!("Total Operations: {}", NUM_OPERATIONS);
    println!("Number of Threads: {}", NUM_THREADS);
    println!("Total Elapsed Time: {:.4} seconds", total_elapsed_seconds);
    println!("Average Time per Operation: {:.8} seconds", total_elapsed_seconds / (NUM_OPERATIONS as f64));
}
