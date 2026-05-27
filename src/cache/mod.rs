/// Caching module for Redis-based storage with input validation
pub mod validation;

pub use validation::CacheValidator;

#[cfg(test)]
mod tests {
    #[test]
    fn test_cache_module_loads() {
        // Module loads successfully
    }
}
