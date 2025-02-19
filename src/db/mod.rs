// Database module entry point (re-exports)
pub mod connection;
pub mod models;
pub mod operations;

pub use connection::get_pool; // Or whatever function you use to get the connection pool
pub use models::*;            // Re-export all models
pub use operations::*;        // Re-export all operations