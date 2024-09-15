use crate::models::common::*;
use crate::schema::bike_trip;
use diesel::prelude::*;
use uuid::Uuid;

/// Represents a bike trip in the database.
#[derive(Debug, Clone, Queryable, Identifiable)]
#[diesel(table_name = bike_trip)]
pub struct BikeTrip {
    /// Unique identifier for the bike trip.
    pub id: String,
    /// Name or description of the bike trip.
    pub name: String,
    /// Optional ID of the bike used for this trip.
    pub bike_id: Option<String>,
}

/// Represents a new bike trip to be inserted into the database.
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = bike_trip)]
pub struct NewBikeTrip {
    /// Unique identifier for the new bike trip.
    pub id: String,
    /// Name or description of the new bike trip.
    pub name: String,
    /// Optional ID of the bike used for this new trip.
    pub bike_id: Option<String>,
}

impl NewBikeTrip {
    /// Creates a new `NewBikeTrip` instance.
    ///
    /// # Arguments
    ///
    /// * `name` - The name or description of the bike trip.
    /// * `bike_id` - An optional ID of the bike used for this trip.
    ///
    /// # Returns
    ///
    /// A new `NewBikeTrip` instance with a generated UUID.
    pub fn new(name: &str, bike_id: Option<&str>) -> Self {
        NewBikeTrip {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            bike_id: bike_id.map(|s| s.to_string()),
        }
    }
}

/// Represents the conditions for filtering bike trips in database queries.
///
/// This enum is crucial for implementing dynamic filtering in the data access layer.
/// It allows for the construction of complex query conditions at runtime,
/// enabling flexible and powerful search capabilities for bike trips.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum Condition {
    /// Filter by the name of the bike trip.
    name(StringFilter),
    /// Filter by conditions related to the associated bike.
    bike(super::bike::Condition),
    /// Combine multiple conditions with a logical AND.
    And(Vec<Condition>),
    /// Combine multiple conditions with a logical OR.
    Or(Vec<Condition>),
}