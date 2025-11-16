use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: f64,
}

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Entity not found")]
    NotFound,
    #[error("Database connection error")]
    ConnectionError,
    #[error("Validation error: {0}")]
    Validation(String),
}