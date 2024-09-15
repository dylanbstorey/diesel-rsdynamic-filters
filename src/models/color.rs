use crate::models::common::*;
use crate::schema::color;
use diesel::prelude::*;
use uuid::Uuid;

/// Represents a color in the database.
#[derive(Debug, Clone, Queryable, Identifiable)]
#[diesel(table_name = color)]
pub struct Color {
    /// Unique identifier for the color.
    pub id: String,
    /// Name of the color.
    pub name: String,
}

/// Represents a new color to be inserted into the database.
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = color)]
pub struct NewColor {
    /// Unique identifier for the new color.
    pub id: String,
    /// Name of the new color.
    pub name: String,
}

impl NewColor {
    /// Creates a new `NewColor` instance.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the color.
    ///
    /// # Returns
    ///
    /// A new `NewColor` instance with a generated UUID.
    pub fn new(name: &str) -> Self {
        NewColor {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
        }
    }
}

/// Represents the conditions for filtering colors in database queries.
///
/// This enum is crucial for implementing dynamic filtering in the data access layer.
/// It allows for the construction of complex query conditions at runtime,
/// enabling flexible and powerful search capabilities for colors.
#[derive(Debug, Clone)]
pub enum ColorCondition {
    /// Filter by the name of the color.
    Name(StringFilter),
    /// Combine multiple conditions with a logical AND.
    And(Vec<ColorCondition>),
    /// Combine multiple conditions with a logical OR.
    Or(Vec<ColorCondition>),
}