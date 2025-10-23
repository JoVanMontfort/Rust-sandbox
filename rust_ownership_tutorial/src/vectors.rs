pub fn run_vector_exercises() {
    println!("=== Rust Vectors Tutorial ===\n");

    // Exercise 1: Basic Vector Operations
    exercise_1_basic_vectors();

    // Exercise 2: Vector Methods
    exercise_2_vector_methods();

    // Exercise 3: Iterating over Vectors
    exercise_3_iterating_vectors();

    // Exercise 4: Vector vs Arrays
    exercise_4_vectors_vs_arrays();

    // Exercise 5: Real-world Examples
    exercise_5_real_world_vectors();

    println!("\n=== Vector Tutorial Complete! ===");
}

// ============================================================================
// EXERCISE 1: Basic Vector Operations
// ============================================================================
fn exercise_1_basic_vectors() {
    println!("1. BASIC VECTOR OPERATIONS");
    println!("{}", "=".repeat(50));

    // Different ways to create vectors
    let v1 = vec![1, 2, 3]; // Using macro
    let v2: Vec<i32> = Vec::new(); // Empty vector with type annotation
    let v3 = Vec::from([1, 2, 3]); // From array

    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
    println!("v3: {:?}", v3);

    // Adding elements (needs to be mutable)
    let mut numbers = Vec::new();
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);
    println!("After push: {:?}", numbers);

    // Accessing elements
    let first = numbers[0]; // Panics if out of bounds
    let second = numbers.get(1); // Returns Option<&i32>

    println!("First: {}, Second: {:?}", first, second);

    // TODO: Create a vector of strings and add some names
    let mut names: Vec<String> = Vec::new();
    names.push(String::from("Alice"));
    names.push(String::from("Bob"));
    println!("Names: {:?}", names);

    println!();
}

// ============================================================================
// EXERCISE 2: Vector Methods
// ============================================================================
fn exercise_2_vector_methods() {
    println!("2. VECTOR METHODS");
    println!("{}", "=".repeat(50));

    let mut scores = vec![85, 92, 78, 96, 88];

    // Common methods
    println!("Length: {}", scores.len());
    println!("Capacity: {}", scores.capacity());
    println!("Is empty: {}", scores.is_empty());

    // Removing elements
    let last_score = scores.pop(); // Remove and return last element
    println!("Popped: {:?}, Remaining: {:?}", last_score, scores);

    scores.insert(1, 95); // Insert at index
    println!("After insert: {:?}", scores);

    let removed = scores.remove(2); // Remove at index
    println!("Removed: {}, Remaining: {:?}", removed, scores);

    scores.clear(); // Remove all elements
    println!("After clear: {:?}", scores);

    // TODO: Practice with these methods
    let mut data = vec![10, 20, 30, 40, 50];
    data.push(60);
    let popped = data.pop();
    data.insert(0, 5);
    println!("Final data: {:?}, Popped: {:?}", data, popped);

    println!();
}

// ============================================================================
// EXERCISE 3: Iterating over Vectors
// ============================================================================
fn exercise_3_iterating_vectors() {
    println!("3. ITERATING OVER VECTORS");
    println!("{}", "=".repeat(50));

    let numbers = vec![1, 2, 3, 4, 5];

    // Method 1: for loop with reference (recommended)
    println!("Method 1 - Immutable references:");
    for number in &numbers {
        println!("Number: {}", number);
    }

    // Method 2: for loop with mutable reference
    let mut scores = vec![10, 20, 30];
    println!("\nMethod 2 - Mutable references:");
    for score in &mut scores {
        *score += 5; // Dereference to modify
        println!("Updated score: {}", score);
    }

    // Method 3: Using iter() explicitly
    println!("\nMethod 3 - Using iter():");
    for number in numbers.iter() {
        println!("Number: {}", number);
    }

    // Method 4: Using into_iter() (consumes the vector)
    println!("\nMethod 4 - Using into_iter():");
    let words = vec!["hello", "world"];
    for word in words.into_iter() {
        println!("Word: {}", word);
    }
    // words is no longer accessible here!

    // TODO: Iterate and modify a vector of prices
    let mut prices = vec![29.99, 49.99, 19.99];
    for price in &mut prices {
        *price *= 1.1; // 10% increase
    }
    println!("Updated prices: {:?}", prices);

    println!();
}

// ============================================================================
// EXERCISE 4: Vector vs Arrays
// ============================================================================
fn exercise_4_vectors_vs_arrays() {
    println!("4. VECTORS vs ARRAYS");
    println!("{}", "=".repeat(50));

    // Arrays - fixed size, stack allocated
    let array: [i32; 3] = [1, 2, 3]; // Type: [i32; 3]
    println!("Array: {:?}, Length: {}", array, array.len());

    // Vectors - dynamic size, heap allocated
    let vector: Vec<i32> = vec![1, 2, 3]; // Type: Vec<i32>
    println!("Vector: {:?}, Length: {}", vector, vector.len());

    // Key differences:
    let mut dynamic_vec = vec![1, 2, 3];
    dynamic_vec.push(4); // Can grow
    dynamic_vec.push(5);
    println!("Grown vector: {:?}", dynamic_vec);

    // Arrays cannot grow!
    // array.push(4);  // COMPILE ERROR: no method named `push`

    // Converting between them
    let array_to_vec = array.to_vec(); // Array to Vector
    let vec_to_array: [i32; 3] = [1, 2, 3]; // Must know exact size

    // TODO: Create both an array and vector with the same data
    let weekdays_array: [&str; 5] = ["Mon", "Tue", "Wed", "Thu", "Fri"];
    let weekdays_vector = vec!["Mon", "Tue", "Wed", "Thu", "Fri"];
    println!("Array: {:?}", weekdays_array);
    println!("Vector: {:?}", weekdays_vector);

    println!();
}

// ============================================================================
// EXERCISE 5: Real-world Examples
// ============================================================================
fn exercise_5_real_world_vectors() {
    println!("5. REAL-WORLD VECTOR EXAMPLES");
    println!("{}", "=".repeat(50));

    // Example 1: Processing user data
    let mut users: Vec<String> = Vec::new();
    users.push("alice@example.com".to_string());
    users.push("bob@example.com".to_string());
    users.push("charlie@example.com".to_string());

    println!("Users: {:?}", users);

    // Example 2: Shopping cart
    #[derive(Debug)]
    struct CartItem {
        name: String,
        price: f64,
        quantity: u32,
    }

    let mut cart: Vec<CartItem> = Vec::new();
    cart.push(CartItem {
        name: "Rust Book".to_string(),
        price: 39.99,
        quantity: 1,
    });
    cart.push(CartItem {
        name: "Mouse".to_string(),
        price: 25.50,
        quantity: 2,
    });

    println!("Cart: {:?}", cart);

    // Example 3: Filtering and mapping
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let even_numbers: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).cloned().collect();

    let squared: Vec<i32> = numbers.iter().map(|x| x * x).collect();

    println!("Even numbers: {:?}", even_numbers);
    println!("Squared: {:?}", squared);

    // TODO: Create a vector of temperatures and find statistics
    let temperatures = vec![72, 68, 75, 80, 65, 70, 78];
    let max_temp = temperatures.iter().max();
    let min_temp = temperatures.iter().min();
    let avg_temp = temperatures.iter().sum::<i32>() / temperatures.len() as i32;

    println!("Temperatures: {:?}", temperatures);
    println!(
        "Max: {:?}, Min: {:?}, Avg: {}",
        max_temp, min_temp, avg_temp
    );

    println!();
}

// ============================================================================
// BONUS: Common Vector Patterns
// ============================================================================
fn bonus_vector_patterns() {
    println!("6. BONUS: COMMON VECTOR PATTERNS");
    println!("{}", "=".repeat(50));

    // Pattern 1: Collecting from iterators
    let squares: Vec<i32> = (1..=5).map(|x| x * x).collect();
    println!("Squares: {:?}", squares);

    // Pattern 2: Using retain to filter in-place
    let mut numbers = vec![1, 2, 3, 4, 5, 6];
    numbers.retain(|&x| x % 2 == 0);
    println!("Even numbers (retained): {:?}", numbers);

    // Pattern 3: Sorting
    let mut names = vec!["Charlie", "Alice", "Bob"];
    names.sort();
    println!("Sorted names: {:?}", names);

    // Pattern 4: Deduplication
    let mut duplicates = vec![1, 2, 2, 3, 4, 4, 5];
    duplicates.dedup();
    println!("Deduplicated: {:?}", duplicates);

    // Pattern 5: Chunks
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    for chunk in data.chunks(3) {
        println!("Chunk: {:?}", chunk);
    }
}
