## Overview
This Rust program generates a public matrix $\boldsymbol{A}\in\mathbb{Z}_q^{m\times n}$, with each element randomly selected from the set $\{0,1,2,\cdots,q-1\}$. The matrix $\boldsymbol{A}$ is commonly used in cryptographic protocols, including ones based on the Learning With Errors (LWE) problem.
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
