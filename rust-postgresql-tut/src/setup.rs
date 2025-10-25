use sqlx::Error;

pub async fn initialize_database() -> Result<(), Error> {
    println!("Initializing database...");

    // Connect to default postgres database first
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect("postgres://triggerIq:triggerIq@localhost/postgres")
        .await?;

    // Try to create database - ignore "already exists" error
    match sqlx::query("CREATE DATABASE triggerIqRust")
        .execute(&pool)
        .await {
        Ok(_) => println!("✅ Database 'triggerIqRust' created"),
        Err(e) => {
            // Check if it's an "already exists" error
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.code().as_deref() == Some("42P04") {
                    println!("✅ Database 'triggerIqRust' already exists");
                } else {
                    return Err(e);
                }
            } else {
                return Err(e);
            }
        }
    }

    // Now connect to our application database
    let app_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect("postgres://triggerIq:triggerIq@localhost/triggerIqRust")
        .await?;

    // Create tables
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            email VARCHAR(100) UNIQUE NOT NULL,
            active BOOLEAN DEFAULT true
        )"
    )
        .execute(&app_pool)
        .await?;

    // Insert sample data
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