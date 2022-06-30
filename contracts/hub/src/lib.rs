#[cfg(not(feature = "library"))]
pub mod contract;

pub mod execute;
pub mod helpers;
pub mod math;
pub mod proto;
pub mod queries;
pub mod state;
pub mod types;

#[cfg(test)]
mod testing;
