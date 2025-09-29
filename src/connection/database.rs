use crate::config::db_config::DbConfig;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;
use tracing::{error, info};

// This module manages database connections.
// Provides functionality to connect to PostgreSQL database and configure connection pools.

/// Establishes and returns a database connection
///
/// This function is called at application startup to connect to the PostgreSQL database.
/// It reads connection information from configuration and sets up the connection pool.
///
/// # Returns
/// * `DatabaseConnection` - Successfully configured database connection object
pub async fn establish_connection() -> DatabaseConnection {
    // Generate database URL from environment configuration
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        &DbConfig::get().db_user,
        &DbConfig::get().db_password,
        &DbConfig::get().db_host,
        &DbConfig::get().db_port,
        &DbConfig::get().db_name
    );
    info!("Attempting to connect to connection: {}", database_url);

    // Configure connection options
    let mut options = ConnectOptions::new(database_url);
    options
        // Connection pool size configuration
        .max_connections(DbConfig::get().db_max_connection) // Maximum connections
        .min_connections(DbConfig::get().db_min_connection) // Minimum connections
        // Timeout configuration
        .connect_timeout(Duration::from_secs(8)) // Connection timeout: 8 seconds
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(300)) // Idle timeout: 5 minutes
        // Enable SQL logging (for debugging)
        .sqlx_logging(false);

    // Attempt database connection and handle result
    match Database::connect(options).await {
        Ok(connection) => {
            // Log success and return connection object
            info!("Successfully connected to the connection.");
            connection
        }
        Err(err) => {
            // Log error and terminate application on connection failure
            // Database connection is a core requirement, cannot proceed without it
            error!("Failed to connect to connection: {}", err);
            panic!("Failed to connect to connection");
        }
    }
}
