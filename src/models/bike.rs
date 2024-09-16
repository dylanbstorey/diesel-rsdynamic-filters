use crate::models::common::*;
use crate::schema::bike;
use diesel::prelude::*;
use uuid::Uuid;

/// Represents a bike in the database.
#[derive(Debug, Clone, Queryable, Identifiable)]
#[diesel(table_name = bike)]
pub struct Bike {
    /// Unique identifier for the bike.
    pub id: String,
    /// Name or description of the bike.
    pub name: String,
    /// Optional ID of the person who owns this bike.
    pub owner_id: Option<String>,
    /// Optional ID of the color of this bike.
    pub color_id: Option<String>,
}

/// Represents a new bike to be inserted into the database.
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = bike)]
pub struct NewBike {
    /// Unique identifier for the new bike.
    pub id: String,
    /// Name or description of the new bike.
    pub name: String,
    /// Optional ID of the person who owns this new bike.
    pub owner_id: Option<String>,
    /// Optional ID of the color of this new bike.
    pub color_id: Option<String>,
}

impl NewBike {
    /// Creates a new `NewBike` instance.
    ///
    /// # Arguments
    ///
    /// * `name` - The name or description of the bike.
    /// * `owner_id` - An optional ID of the person who owns this bike.
    /// * `color_id` - An optional ID of the color of this bike.
    ///
    /// # Returns
    ///
    /// A new `NewBike` instance with a generated UUID.
    pub fn new(name: &str, owner_id: Option<&str>, color_id: Option<&str>) -> Self {
        NewBike {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            owner_id: owner_id.map(|s| s.to_string()),
            color_id: color_id.map(|s| s.to_string()),
        }
    }
}

/// Represents the conditions for filtering bikes in database queries.
///
/// This enum is crucial for implementing dynamic filtering in the data access layer.
/// It allows for the construction of complex query conditions at runtime,
/// enabling flexible and powerful search capabilities for bikes.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum BikeCondition {
    /// Filter by the name of the bike.
    name(StringFilter),
    /// Filter by the color of the bike.
    color(StringFilter),
    /// Combine multiple conditions with a logical AND.
    And(Vec<BikeCondition>),
    /// Combine multiple conditions with a logical OR.
    Or(Vec<BikeCondition>),
}