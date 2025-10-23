pub fn run_type_exercises() {
    println!("=== Rust Type System Tutorial ===\n");

    // Exercise 1: Basic Type Inference
    exercise_1_basic_types();

    // Exercise 2: Type Annotations
    exercise_2_type_annotations();

    // Exercise 3: Complex Types
    exercise_3_complex_types();

    // Exercise 4: Type Conversion
    exercise_4_type_conversion();

    // Exercise 5: Generics
    exercise_5_generics();

    // Exercise 6: Traits and Polymorphism
    exercise_6_traits();

    println!("\n=== Type System Mastered! ===");
}

// ============================================================================
// EXERCISE 1: Basic Type Inference
// ============================================================================
fn exercise_1_basic_types() {
    println!("1. BASIC TYPE INFERENCE");
    println!("{}", "=".repeat(50));

    // Rust infers types from usage
    let a = 42; // i32
    let b = 42.0; // f64  
    let c = true; // bool
    let d = 'R'; // char
    let e = "Rust"; // &str
    let f = String::from("Rust"); // String

    // TODO: What types do these have? Uncomment to check:
    // println!("Type of a: {}", std::any::type_name_of_val(&a));
    // println!("Type of b: {}", std::any::type_name_of_val(&b));
    // println!("Type of c: {}", std::any::type_name_of_val(&c));
    // println!("Type of d: {}", std::any::type_name_of_val(&d));
    // println!("Type of e: {}", std::any::type_name_of_val(&e));
    // println!("Type of f: {}", std::any::type_name_name_of_val(&f));

    // Type mismatches cause compile errors
    // let sum = a + b;  // ERROR: i32 + f64

    println!("Basic types work correctly!");
    println!();
}

// ============================================================================
// EXERCISE 2: Type Annotations
// ============================================================================
fn exercise_2_type_annotations() {
    println!("2. EXPLICIT TYPE ANNOTATIONS");
    println!("{}", "=".repeat(50));

    // Sometimes you need to be explicit
    let x: i32 = 10;
    let y: f64 = 3.14;
    let is_rust_cool: bool = true;
    let grade: char = 'A';
    let message: &str = "Hello, Rust!";
    let dynamic_string: String = String::from("Dynamic");

    // Collections with type annotations
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    let array: [i32; 3] = [10, 20, 30]; // Fixed-size array
    let tuple: (i32, f64, bool) = (500, 6.4, true);

    // TODO: Add type annotations to these variables
    let score = 100; // Should be u8
    let temperature = -5; // Should be i8  
    let distance = 1234.56; // Should be f32
    let items = vec!["apple", "banana"]; // Should be Vec<&str>

    println!("Tuple: ({}, {}, {})", tuple.0, tuple.1, tuple.2);
    println!("Array: {:?}", array);
    println!();
}

// ============================================================================
// EXERCISE 3: Complex Types
// ============================================================================
fn exercise_3_complex_types() {
    println!("3. COMPLEX TYPES");
    println!("{}", "=".repeat(50));

    // Option type - represents optional values
    let some_number: Option<i32> = Some(5);
    let no_number: Option<i32> = None;

    // Result type - for error handling
    let success: Result<i32, String> = Ok(42);
    let failure: Result<i32, String> = Err(String::from("Something went wrong"));

    // Vectors (dynamic arrays)
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    // HashMaps
    use std::collections::HashMap;
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    // TODO: Create these complex types:
    // 1. An Option that might contain a String
    // 2. A Result that could be a f64 or a String error
    // 3. A HashMap mapping &str to bool

    println!("Some: {:?}, None: {:?}", some_number, no_number);
    println!("Success: {:?}, Failure: {:?}", success, failure);
    println!("Numbers: {:?}", numbers);
    println!("Scores: {:?}", scores);
    println!();
}

// ============================================================================
// EXERCISE 4: Type Conversion
// ============================================================================
fn exercise_4_type_conversion() {
    println!("4. TYPE CONVERSION");
    println!("{}", "=".repeat(50));

    // Explicit casting
    let integer: i32 = 100;
    let decimal: f64 = integer as f64; // i32 to f64
    let small_int: i8 = integer as i8; // i32 to i8 (may lose data)

    // String conversion
    let num_str = "42";
    let number: i32 = num_str.parse().expect("Not a number!");
    let number_alt: i32 = num_str.parse().unwrap(); // Simpler but panics on error

    // Between String and &str
    let string_owned = String::from("hello");
    let string_slice: &str = &string_owned;
    let back_to_owned: String = string_slice.to_string();

    // TODO: Fix these conversion errors
    let big_number: i64 = 1_000_000;
    // let small_number: i32 = big_number;  // ERROR: i64 to i32
    let small_number: i32 = big_number as i32; // Fixed with cast

    let text = "123";
    // let parsed: i32 = text;  // ERROR: &str to i32
    let parsed: i32 = text.parse().unwrap(); // Fixed with parse

    println!("Casting: {} -> {} -> {}", integer, decimal, small_int);
    println!("Parsing: '{}' -> {}", num_str, number);
    println!("String conversions work!");
    println!();
}

// ============================================================================
// EXERCISE 5: Generics
// ============================================================================
fn exercise_5_generics() {
    println!("5. GENERICS");
    println!("{}", "=".repeat(50));

    // Generic struct
    struct Point<T> {
        x: T,
        y: T,
    }

    // Can create points with different types
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    // Generic functions
    fn get_first<T>(slice: &[T]) -> Option<&T> {
        slice.first()
    }

    let numbers = vec![1, 2, 3];
    let first_num = get_first(&numbers);

    let words = vec!["hello", "world"];
    let first_word = get_first(&words);

    // TODO: Create a generic function that returns the last element
    // fn get_last<T>(...) -> ... { ... }

    println!("Integer point: ({}, {})", integer_point.x, integer_point.y);
    println!("Float point: ({}, {})", float_point.x, float_point.y);
    println!("First number: {:?}", first_num);
    println!("First word: {:?}", first_word);
    println!();
}

// ============================================================================
// EXERCISE 6: Traits and Polymorphism
// ============================================================================
fn exercise_6_traits() {
    println!("6. TRAITS AND POLYMORPHISM");
    println!("{}", "=".repeat(50));

    // Define a trait
    trait Describable {
        fn describe(&self) -> String;

        fn default_description(&self) -> String {
            String::from("This is a describable object")
        }
    }

    // Implement trait for different types
    struct Person {
        name: String,
        age: u8,
    }

    struct Car {
        model: String,
        year: u16,
    }

    impl Describable for Person {
        fn describe(&self) -> String {
            format!("{} is {} years old", self.name, self.age)
        }
    }

    impl Describable for Car {
        fn describe(&self) -> String {
            format!("{} model from {}", self.model, self.year)
        }
    }

    // Trait bounds in functions
    fn print_description<T: Describable>(item: &T) {
        println!("{}", item.describe());
    }

    let alice = Person {
        name: String::from("Alice"),
        age: 30,
    };
    let mustang = Car {
        model: String::from("Mustang"),
        year: 2020,
    };

    print_description(&alice);
    print_description(&mustang);

    // TODO: Implement the Describable trait for this struct
    struct Book {
        title: String,
        author: String,
    }

    // impl Describable for Book { ... }

    println!();
}

// ============================================================================
// BONUS: Advanced Type Features
// ============================================================================
fn advanced_types() {
    println!("7. ADVANCED TYPE FEATURES");
    println!("{}", "=".repeat(50));

    // Type aliases
    type UserId = u64;
    type Result<T> = std::result::Result<T, String>;

    let user_id: UserId = 12345;
    let computation: Result<i32> = Ok(42);

    // Never type (!) - for functions that never return
    fn forever() -> ! {
        loop {
            println!("This runs forever!");
        }
    }

    // Sized and Unsized types
    fn process_sized<T: Sized>(_t: T) {
        // T must have known size at compile time
    }

    // TODO: Explore these concepts further:
    // - PhantomData
    // - Associated types
    // - Higher-ranked trait bounds (HRTB)

    println!("User ID: {}", user_id);
    println!("Result: {:?}", computation);
    println!("Advanced types are powerful!");
}

// Helper function to demonstrate type names
fn print_type_of<T>(_: &T) {
    println!("Type: {}", std::any::type_name::<T>());
}
