use std::rc::Rc;
use std::sync::Arc;

pub fn run_ownership_exercises() {
    println!("=== Rust Ownership & Borrowing Tutorial ===\n");

    // Exercise 1: Understanding Basic Ownership
    exercise_1_basic_ownership();

    // Exercise 2: References vs Copies
    exercise_2_references_vs_copies();

    // Exercise 3: Mutable References
    exercise_3_mutable_references();

    // Exercise 4: String vs &str
    exercise_4_string_vs_str();

    // Exercise 5: Real-world Scenarios
    exercise_5_real_world_scenarios();

    // Exercise 6: Advanced Topics
    exercise_6_advanced_topics();

    println!("\n=== Tutorial Complete! ===");
}

// ============================================================================
// EXERCISE 1: Basic Ownership
// ============================================================================
fn exercise_1_basic_ownership() {
    println!("1. BASIC OWNERSHIP");
    println!("{}", "=".repeat(50));

    // Scenario 1: Simple integers (Copy trait)
    let x = 5;
    let y = x; // Copy happens here
    println!("x = {}, y = {}", x, y); // Both still usable

    // Scenario 2: String (no Copy trait)
    let s1 = String::from("hello");
    let s2 = s1; // Move happens here, s1 is no longer valid
    // println!("s1 = {}", s1);  // UNCOMMENT THIS LINE TO SEE ERROR
    println!("s2 = {}", s2);

    // Scenario 3: Explicit clone
    let s3 = String::from("world");
    let s4 = s3.clone(); // Explicit copy
    println!("s3 = {}, s4 = {}", s3, s4); // Both still usable

    // TODO: Fix the error in this function
    problematic_function();

    println!();
}

fn problematic_function() {
    let data = String::from("important data");
    let result = process_and_return(&data);
    println!("Original data: {}", data); // FIXME: This line causes error
    println!("Processed result: {}", result);
}

fn process_and_return(s: &String) -> String {
    format!("Processed: {}", s)
}

// ============================================================================
// EXERCISE 2: References vs Copies
// ============================================================================
fn exercise_2_references_vs_copies() {
    println!("2. REFERENCES vs COPIES");
    println!("{}", "=".repeat(50));

    let large_data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Method 1: Pass by reference (efficient)
    let sum_ref = sum_by_reference(&large_data);
    println!("Sum by reference: {}", sum_ref);
    println!("Original data still available: {:?}", large_data);

    // Method 2: Pass by value (inefficient - copy)
    let sum_copy = sum_by_copy(large_data.clone()); // Explicit copy
    println!("Sum by copy: {}", sum_copy);
    println!("Original data still available: {:?}", large_data);

    // TODO: Implement this function efficiently
    let numbers = vec![10, 20, 30, 40, 50];
    let average = calculate_average(&numbers); // Should use reference
    println!("Average of {:?} = {}", numbers, average);

    println!();
}

fn sum_by_reference(data: &[i32]) -> i32 {
    data.iter().sum()
}

fn sum_by_copy(data: Vec<i32>) -> i32 {
    data.iter().sum()
}

// TODO: Implement this function using references
fn calculate_average(numbers: &[i32]) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }
    let sum: i32 = numbers.iter().sum();
    sum as f64 / numbers.len() as f64
}

// ============================================================================
// EXERCISE 3: Mutable References
// ============================================================================
fn exercise_3_mutable_references() {
    println!("3. MUTABLE REFERENCES");
    println!("{}", "=".repeat(50));

    let mut scores = vec![85, 92, 78, 96];
    println!("Original scores: {:?}", scores);

    // Immutable reference - can have multiple
    let first_score = &scores[0];
    let second_score = &scores[1];
    println!("First: {}, Second: {}", first_score, second_score);

    // Mutable reference - exclusive access
    add_bonus_points(&mut scores, 5);
    println!("After bonus: {:?}", scores);

    // TODO: Fix the borrowing conflict in this function
    borrowing_conflict_example();

    println!();
}

fn add_bonus_points(scores: &mut Vec<i32>, bonus: i32) {
    for score in scores {
        *score += bonus;
    }
}

fn borrowing_conflict_example() {
    let mut data = vec![1, 2, 3];

    let immutable_ref = &data[0]; // Immutable borrow
    // let mutable_ref = &mut data[1];  // FIXME: Uncommenting causes error
    println!("Immutable reference: {}", immutable_ref);

    // Mutable borrow can happen here after immutable borrow is done
    let mutable_ref = &mut data[1];
    *mutable_ref = 99;
    println!("After mutable change: {:?}", data);
}

// ============================================================================
// EXERCISE 4: String vs &str
// ============================================================================
fn exercise_4_string_vs_str() {
    println!("4. String vs &str");
    println!("{}", "=".repeat(50));

    // String - owned, heap-allocated, growable
    let owned_string = String::from("Hello, World!");

    // &str - string slice, reference to string data
    let string_slice: &str = &owned_string;
    let literal_slice: &str = "I'm a string literal";

    process_string_reference(string_slice);
    process_string_reference(literal_slice);

    // TODO: Choose the right parameter type for these functions
    let name = String::from("Alice");
    greet_correctly(&name); // Should use reference
    greet_correctly("Bob"); // String literal works too

    println!();
}

fn process_string_reference(s: &str) {
    println!("Processing string slice: {}", s);
}

// TODO: Fix the function signature to accept both String and &str efficiently
fn greet_correctly(name: &str) {
    println!("Hello, {}!", name);
}

// ============================================================================
// EXERCISE 5: Real-world Scenarios
// ============================================================================
fn exercise_5_real_world_scenarios() {
    println!("5. REAL-WORLD SCENARIOS");
    println!("{}", "=".repeat(50));

    // Scenario 1: Configuration data
    let config = ServerConfig {
        host: String::from("localhost"),
        port: 8080,
        timeout: 30,
    };

    start_server(&config);
    validate_config(&config); // Can use config multiple times

    // Scenario 2: Data processing pipeline
    let mut user_data = UserData {
        name: String::from("John Doe"),
        age: 30,
        scores: vec![85, 92, 78],
    };

    // Pass mutable reference for modification
    update_user_age(&mut user_data, 31);
    add_user_score(&mut user_data, 95);

    println!("Updated user: {:?}", user_data);

    // TODO: Implement this function
    let analysis = analyze_user_data(&user_data);
    println!("User analysis: {}", analysis);

    println!();
}

#[derive(Debug)]
struct ServerConfig {
    host: String,
    port: u16,
    timeout: u32,
}

#[derive(Debug)]
struct UserData {
    name: String,
    age: u8,
    scores: Vec<i32>,
}

fn start_server(config: &ServerConfig) {
    println!("Starting server at {}:{}", config.host, config.port);
}

fn validate_config(config: &ServerConfig) -> bool {
    !config.host.is_empty() && config.port > 0
}

fn update_user_age(user: &mut UserData, new_age: u8) {
    user.age = new_age;
}

fn add_user_score(user: &mut UserData, score: i32) {
    user.scores.push(score);
}

// TODO: Implement this function using references
fn analyze_user_data(user: &UserData) -> String {
    let avg_score = if user.scores.is_empty() {
        0.0
    } else {
        user.scores.iter().sum::<i32>() as f64 / user.scores.len() as f64
    };

    format!(
        "{} (age {}) has average score: {:.2}",
        user.name, user.age, avg_score
    )
}

// ============================================================================
// EXERCISE 6: Advanced Topics
// ============================================================================
fn exercise_6_advanced_topics() {
    println!("6. ADVANCED TOPICS");
    println!("{}", "=".repeat(50));

    // Reference counting for shared ownership
    let shared_data = Rc::new(String::from("Shared between components"));
    let component_a = shared_data.clone();
    let component_b = shared_data.clone();
    println!("Reference count: {}", Rc::strong_count(&shared_data));

    // Thread-safe reference counting
    let arc_data = Arc::new(vec![1, 2, 3]);
    let thread_data = arc_data.clone();
    println!("Arc data: {:?}", thread_data);

    // TODO: Fix the lifetime issue in this function
    let result;
    {
        let temporary_string = String::from("temporary data");
        result = create_string_slice(&temporary_string);
        println!("String slice: {}", result);
    }
    // println!("Outside: {}", result);  // FIXME: This will cause error

    println!();
}

fn create_string_slice(s: &str) -> &str {
    &s[0..5] // Returns a slice of the input
}

// ============================================================================
// ADDITIONAL PRACTICE EXERCISES
// ============================================================================
fn additional_exercises() {
    println!("7. ADDITIONAL PRACTICE");
    println!("{}", "=".repeat(50));

    // Exercise A: Fix the function to use references properly
    let items = vec!["apple", "banana", "cherry"];
    let filtered = filter_items(&items, "a");
    println!("Filtered items: {:?}", filtered);
    println!("Original items still available: {:?}", items);

    // Exercise B: Implement a function that modifies a collection in place
    let mut numbers = vec![1, 2, 3, 4, 5];
    double_values(&mut numbers);
    println!("Doubled numbers: {:?}", numbers);

    println!();
}

fn filter_items<'a>(items: &'a [&'a str], pattern: &str) -> Vec<&'a str> {
    items
        .iter()
        .filter(|&&item| item.contains(pattern))
        .copied()
        .collect()
}

fn double_values(numbers: &mut Vec<i32>) {
    for num in numbers {
        *num *= 2;
    }
}
