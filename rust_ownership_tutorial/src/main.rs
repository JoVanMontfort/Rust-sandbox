mod vectors;
mod ownership;
mod types;

fn main() {
    println!("=== Rust Tutorial Collection ===\n");

    // Run vector exercises
    vectors::run_vector_exercises();

    // Run ownership exercises
    ownership::run_ownership_exercises();

    // Run type system exercises
    types::run_type_exercises();

    println!("\n=== All Tutorials Complete! ===");
}