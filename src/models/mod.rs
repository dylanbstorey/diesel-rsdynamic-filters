pub mod person;
pub mod bike;
pub mod color;
pub mod bike_trip;


// Common types and enums
pub mod common {
    #[derive(Debug, Clone)]
    pub enum StringFilter {
        Equal(String),
        NotEqual(String),
        Like(String),
        In(Vec<String>),
    }

    #[derive(Debug, Clone)]
    pub enum NumberFilter<T> {
        Equal(T),
        NotEqual(T),
        GreaterThen(T),
        LowerThen(T),
        IsNull,
        IsNotNull,
    }

    #[derive(Debug, Clone)]
    pub enum BooleanFilter {
        True,
        False,
        IsNull,
        IsNotNull,
    }

    #[derive(Debug, Clone)]
    pub enum AndOr {
        And,
        Or,
    }
}

// Re-export common types for easier access
pub use self::common::*;