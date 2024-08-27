use rand::Rng;

// Function to generate a random matrix A in Z_q^{m x n}
fn generate_public_matrix(q: u64, m: usize, n: usize) -> Vec<Vec<u64>> {
    let mut rng = rand::thread_rng();
    let mut matrix = Vec::with_capacity(m);

    for _ in 0..m {
        let mut row = Vec::with_capacity(n);
        for _ in 0..n {
            // Generate a random number in the range [0, q-1]
            row.push(rng.gen_range(0..q));
        }
        matrix.push(row);
    }

    matrix
}

fn main() {
    // Example parameters
    let q = 13;  // Modulus
    let m = 4;   // Number of rows
    let n = 3;   // Number of columns

    let matrix = generate_public_matrix(q, m, n);
    println!("Public Matrix A:");
    for row in matrix {
        println!("{:?}", row);
    }
}

