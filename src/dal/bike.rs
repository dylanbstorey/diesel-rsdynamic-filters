use diesel::pg::Pg;
use diesel::{
    helper_types::{IntoBoxed, LeftJoin, LeftJoinQuerySource},
    prelude::*,
    sql_types::{Bool, Nullable},
};
use diesel::r2d2::{self, ConnectionManager};
use crate::models::bike::{Bike, NewBike};
use crate::models::common::StringFilter;
use crate::schema;
use crate::schema::bike::dsl::*;
use crate::models::bike::BikeCondition;
use crate::dal::string_filter;
use crate::models::AndOr;

/// Type alias for the database connection pool
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

type ConditionSource = LeftJoinQuerySource<schema::bike::dsl::bike, schema::color::dsl::color>;
type BoxedCondition = Box<dyn BoxableExpression<ConditionSource, Pg, SqlType = Nullable<Bool>>>;
type QuerySource = LeftJoin<schema::bike::dsl::bike, schema::color::dsl::color>;
type BoxedQuery = IntoBoxed<'static, QuerySource, Pg>;

impl BikeCondition {
    fn to_boxed_condition(self) -> Option<BoxedCondition> {
        Some(match self {
            BikeCondition::name(f) => string_filter!(f, schema::bike::dsl::name),
            BikeCondition::color(f) => string_filter!(f, schema::color::dsl::name),
            BikeCondition::And(conditions) => match create_filter(conditions, AndOr::And) {
                Some(boxed_condition) => boxed_condition,
                None => return None,
            },
            BikeCondition::Or(conditions) => match create_filter(conditions, AndOr::Or) {
                Some(boxed_condition) => boxed_condition,
                None => return None,
            },
        })
    }
}


// This method can also be made into a macro, but it should be fine to just duplicate
fn create_filter(conditions: Vec<BikeCondition>, and_or: AndOr) -> Option<BoxedCondition> {
    conditions
        .into_iter()
        // Map into array of boxed conditions
        .filter_map::<BoxedCondition, _>(BikeCondition::to_boxed_condition)
        // Reduce to a boxed_condition1.and(boxed_condition2).and(boxed_condition3)...
        .fold(None, |boxed_conditions, boxed_condition| {
            Some(match boxed_conditions {
                Some(bc) => match and_or {
                    AndOr::And => Box::new(bc.and(boxed_condition)),
                    AndOr::Or => Box::new(bc.or(boxed_condition)),
                },
                None => boxed_condition,
            })
        })
}
pub(super) fn create_filtered_query(conditions: Vec<BikeCondition>) -> BoxedQuery {
    let boxed_query = schema::bike::dsl::bike.left_join(schema::color::dsl::color).into_boxed();

    match create_filter(conditions, AndOr::And) {
        Some(boxed_conditions) => boxed_query.filter(boxed_conditions),
        None => boxed_query,
    }
}


/// Data Access Layer for Bike entities
pub struct BikeDAL {
    pool: Pool,
}

impl BikeDAL {
    /// Creates a new BikeDAL instance
    ///
    /// # Arguments
    ///
    /// * `pool` - The database connection pool
    pub fn new(pool: Pool) -> Self {
        BikeDAL { pool }
    }

    /// Creates a new bike in the database
    ///
    /// # Arguments
    ///
    /// * `new_bike` - The new bike to be created
    ///
    /// # Returns
    ///
    /// The created bike or a database error
    pub fn create(&self, new_bike: &NewBike) -> QueryResult<Bike> {
        let mut conn = self.pool.get().expect("Couldn't get DB connection");
        diesel::insert_into(bike)
            .values(new_bike)
            .get_result(&mut conn)
    }

    /// Finds a bike by its ID
    ///
    /// # Arguments
    ///
    /// * `bike_id` - The ID of the bike to find
    ///
    /// # Returns
    ///
    /// The found bike or a database error
    pub fn find_by_id(&self, bike_id: &str) -> QueryResult<Bike> {
        let mut conn = self.pool.get().expect("Couldn't get DB connection");
        bike.find(bike_id).first(&mut conn)
    }

    /// Retrieves all bikes from the database
    ///
    /// # Returns
    ///
    /// A vector of all bikes or a database error
    pub fn find_all(&self) -> QueryResult<Vec<Bike>> {
        let mut conn = self.pool.get().expect("Couldn't get DB connection");
        bike.load::<Bike>(&mut conn)
    }

    /// Updates an existing bike in the database
    ///
    /// # Arguments
    ///
    /// * `bike_id` - The ID of the bike to update
    /// * `updated_bike` - The updated bike data
    ///
    /// # Returns
    ///
    /// The updated bike or a database error
    pub fn update(&self, bike_id: &str, updated_bike: &Bike) -> QueryResult<Bike> {
        let mut conn = self.pool.get().expect("Couldn't get DB connection");
        diesel::update(bike.find(bike_id))
            .set((
                name.eq(&updated_bike.name),
                owner_id.eq(&updated_bike.owner_id),
                color_id.eq(&updated_bike.color_id),
            ))
            .get_result(&mut conn)
    }

    /// Deletes a bike from the database
    ///
    /// # Arguments
    ///
    /// * `bike_id` - The ID of the bike to delete
    ///
    /// # Returns
    ///
    /// The number of affected rows or a database error
    pub fn delete(&self, bike_id: &str) -> QueryResult<usize> {
        let mut conn = self.pool.get().expect("Couldn't get DB connection");
        diesel::delete(bike.find(bike_id))
            .execute(&mut conn)
    }

    /// Finds bikes with filters using Condition
    ///
    /// # Arguments
    ///
    /// * `conditions` - A vector of Condition enums for filtering
    ///
    /// # Returns
    ///
    /// A vector of bikes matching the filters or a database error
    pub fn find_with_filters(&self, conditions: Vec<BikeCondition>) -> QueryResult<Vec<Bike>> {
        let mut conn = self.pool.get().expect("Couldn't get DB connection");
        
        let query = create_filtered_query(conditions);

        query
            .select(bike::all_columns())
            .distinct()
            .load::<Bike>(&mut conn)
    }  
}

