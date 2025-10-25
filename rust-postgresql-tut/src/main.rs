mod setup;

use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    match setup::initialize_database().await {
        Ok(()) => println!("✅ Database setup completed successfully!"),
        Err(e) => {
            eprintln!("❌ Database setup failed: {}", e);
            return Err(e.into());
        }
    }

    // Create connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://triggerIq:triggerIq@localhost/triggerIqRust")
        .await?;

    // Execute a simple query
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 150);

    // Query with type checking at compile time
    #[derive(sqlx::FromRow)]
    struct User {
        id: i32,
        name: String,
        email: String,
    }

    let users = sqlx::query_as::<_, User>("SELECT id, name, email FROM users WHERE active = $1")
        .bind(true)
        .fetch_all(&pool)
        .await?;

    for user in users {
        println!("User: {} - {} - {}", user.id, user.name, user.email);
    }

    Ok(())
}
