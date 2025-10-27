use sqlx::Error;
use tokio::time::{sleep, Duration};

pub async fn initialize_database() -> Result<(), Error> {
    println!("Initializing database...");

    // Connect to default postgres database first
    let admin_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect("postgres://triggerIq:triggerIq@localhost/postgres")
        .await?;

    // Drop and recreate database
    println!("🗑️ Dropping existing database...");
    let _ = sqlx::query("DROP DATABASE IF EXISTS \"triggerIqRust\"")
        .execute(&admin_pool)
        .await;

    println!("🔄 Creating fresh database...");
    sqlx::query("CREATE DATABASE \"triggerIqRust\"")
        .execute(&admin_pool)
        .await?;

    // Verify creation - use lowercase for system catalog
    let db_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM pg_database WHERE datname = 'triggeriqrust')"
    )
        .fetch_one(&admin_pool)
        .await?;

    if !db_exists {
        return Err(Error::Configuration("Failed to create database".into()));
    }
    println!("✅ Database verified in system catalog!");

    // Wait for initialization
    println!("⏳ Waiting for database to initialize...");
    sleep(Duration::from_secs(3)).await;

    // Close the admin pool
    drop(admin_pool);

    // Now connect to the new database - FIXED VERSION
    println!("🔄 Connecting to application database...");
    let app_pool = match sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect("postgres://triggerIq:triggerIq@localhost/triggerIqRust")
        .await {
        Ok(pool) => pool,
        Err(_) => {
            println!("⚠️  First connection failed, trying alternative...");
            sqlx::postgres::PgPoolOptions::new()
                .max_connections(1)
                .connect("postgres://triggerIq:triggerIq@localhost/triggeriqrust")
                .await?
        }
    };

    println!("✅ Successfully connected to application database!");

    // Create tables
    println!("🔄 Creating tables...");
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            email VARCHAR(100) UNIQUE NOT NULL,
            active BOOLEAN DEFAULT true,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
        .execute(&app_pool)
        .await?;

    // Insert sample data
    println!("🔄 Inserting sample data...");
    sqlx::query(
        "INSERT INTO users (name, email) VALUES
        ($1, $2), ($3, $4), ($5, $6)
        ON CONFLICT (email) DO NOTHING"
    )
        .bind("Alice Smith")
        .bind("alice@example.com")
        .bind("Bob Johnson")
        .bind("bob@example.com")
        .bind("Carol Davis")
        .bind("carol@example.com")
        .execute(&app_pool)
        .await?;

    println!("✅ Database setup completed successfully!");
    Ok(())
}