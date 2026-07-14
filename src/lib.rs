//! Convergence Engine - Sovereign AI Framework
//! Human intent + AI cognition + gated execution

pub mod intent;
pub mod cognition;
pub mod execution;

pub const VERSION: &str = "1.0.0";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(VERSION, "1.0.0");
    }
}