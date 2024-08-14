// Function to find elements of Z_n
fn zn(n: u32) -> Vec<u32> {
    (0..n).collect()
}

// Function to compute the direct product of Z_n and Z_m
fn direct_product(n: u32, m: u32) -> Vec<(u32, u32)> {
    let zn_elements = zn(n);
    let zm_elements = zn(m);
    
    let mut product = Vec::new();
    for &a in &zn_elements {
        for &b in &zm_elements {
            product.push((a, b)); 
        }
    }
    product
}

// Function to compute the sum (a, b) + (e, f) mod (n, m)
fn sum_mod((a, b): (u32, u32), (e, f): (u32, u32), n: u32, m: u32) -> (u32, u32) {
    ((a + e) % n, (b + f) % m)
}

// Function to compute the inverse of (t, s) in Z_n x Z_m
fn inverse_mod((t, s): (u32, u32), n: u32, m: u32) -> (u32, u32) {
    ((n - t) % n, (m - s) % m)
}

fn main() {
    let n = 10;
    let m = 8;

    let elements = direct_product(n, m);

    println!("Direct product of Z_{} and Z_{}:", n, m);
    for (a, b) in &elements {
        println!("({}, {})", a, b);
    }

    // Example of computing (a, b) + (e, f)
    let (a, b) = elements[12]; // Example element from Z_n x Z_m
    let (e, f) = elements[11]; // Another example element from Z_n x Z_m

    let sum = sum_mod((a, b), (e, f), n, m);

    println!("\nSum of ({}, {}) and ({}, {}) mod ({}, {}): ({}, {})", a, b, e, f, n, m, sum.0, sum.1);

    // Example of computing inverse of (t, s)
    let (t, s) = elements[13]; // Example element from Z_n x Z_m
    let inverse = inverse_mod((t, s), n, m);

    println!("\nInverse of ({}, {}) in Z_{} x Z_{} is: ({}, {})", t, s, n, m, inverse.0, inverse.1);
}

