use crate::models::common::*;
use crate::schema::person;
use diesel::prelude::*;
use uuid::Uuid;

/// Represents a person in the database.
#[derive(Debug, Clone, Queryable, Identifiable)]
#[diesel(table_name = person)]
pub struct Person {
    /// Unique identifier for the person.
    pub id: String,
    /// Name of the person.
    pub name: String,
}

/// Represents a new person to be inserted into the database.
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = person)]
pub struct NewPerson {
    /// Unique identifier for the new person.
    pub id: String,
    /// Name of the new person.
    pub name: String,
}

impl NewPerson {
    /// Creates a new `NewPerson` instance.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the person.
    ///
    /// # Returns
    ///
    /// A new `NewPerson` instance with a generated UUID.
    pub fn new(name: &str) -> Self {
        NewPerson {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
        }
    }
}

/// Represents the conditions for filtering persons in database queries.
///
/// This enum is crucial for implementing dynamic filtering in the data access layer.
/// It allows for the construction of complex query conditions at runtime,
/// enabling flexible and powerful search capabilities for persons.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum PersonCondition {
    /// Filter by the name of the person.
    name(StringFilter),
    /// Filter by conditions related to the bikes owned by the person.
    bike(Vec<crate::models::bike::BikeCondition>),
    /// Combine multiple conditions with a logical AND.
    And(Vec<PersonCondition>),
    /// Combine multiple conditions with a logical OR.
    Or(Vec<PersonCondition>),
}