## Overview
This Rust program generates a public matrix $\boldsymbol{A}\in\mathbb{Z}_q^{m\times n}$, with each element randomly selected from the set {0,1,2,\cdots,q-1}. The matrix $\boldsymbol{A}$ is commonly used in cryptographic protocols, including ones based on the Learning With Errors (LWE) problem. This code is an educational example of random matrix generation and its use in cryptography.
## How It Works
- **Parameters:**
   - **Modulus $q$:** A prime number or any integer greater than $1$.
   - **Rows $m$:** The number of rows in the matrix.
   - **Columns $n$:** The number of columns in the matrix.
- **Matrix Generation:**
   - The matrix is generated with $m$ rows and $n$ columns.
   - Each element of the matrix is a random integer between $0$ and $q-1$, inclusive.
- **Rust Functionality:**
   - The program uses the rand crate to generate random numbers.
   - The function generate_public_matrix creates and returns the matrix as a Vec<Vec<u64>>, where each Vec<u64> represents a row in the matrix.
## Contributing
  - If you intend to contribute to this project, fork the repository and make a pull request.

  ## Installation

- To use this project, you need to have Rust installed on your machine.
- If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install it.
- After installing Rust, clone this repository or copy the code into a Rust project, Compile and run the code using cargo run.
- Ensure you have the rand crate in your Cargo.toml file:
 >```
 > [dependencies]
 >rand = "0.8"  # Or the latest version
## Code Explanation
- **Main function**
   - The main function initializes the matrix dimensions and modulus.
   - It then calls **generate_public_matrix(q, m, n)** to create the matrix.
   - Prints the matrix is row by row.
- **Random Matrix Generation:**
   - **rng.gen_range(0..q)** generates a random integer in the range $[0, q)$.
   - This process is repeated for every element in the matrix.
## Usage
- To use this code, you can clone the repository.
- You can change the values of $q,n$ and $m$ in the main function to test different cases.
- Compile the Rust code using cargo:
>```
>cargo build
>cargo run
- This will compile and run the program, printing a public matrix for the provided parameters.
## Example Usage
>```
> fn main() {
> // Example parameters
>   let q = 13;  // Modulus
>   let m = 4;   // Number of rows
>   let n = 3;   // Number of columns
>  let matrix = generate_public_matrix(q, m, n);
> println!("Public Matrix A:");
>for row in matrix {
>     println!("{:?}", row);
> }
>}

## Example Output
- The program will output a 4x3 matrix with random elements from the set {0,1,…,12}.
>```
>Public Matrix A:
>[5, 8, 3]
>[0, 11, 4]
>[7, 2, 9]
>[10, 6, 1]

 
## Acknowledgments
- Rust
### Clone the repository or copy the source code into a Rust project.
```bash
   git clone https://github.com/cypriansakwa/Random_Public_Matrix_Generation_Using_Rust_for_Cryptographic_Applications.git
   cd Random_Public_Matrix_Generation_Using_Rust_for_Cryptographic_Applications
