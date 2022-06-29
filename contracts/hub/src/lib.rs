#[cfg(not(feature = "library"))]
pub mod contract;

pub mod error;
pub mod execute;
pub mod helpers;
pub mod math;
pub mod queries;
pub mod state;
pub mod types;
pub mod vault_token;

#[cfg(test)]
mod testing;
