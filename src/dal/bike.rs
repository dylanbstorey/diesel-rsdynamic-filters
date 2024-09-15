use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use crate::models::bike::{Bike, NewBike};
use crate::models::common::{StringFilter, NumberFilter};
use crate::schema::bike::dsl::*;

/// Type alias for the database connection pool
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

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

    /// Finds bikes with optional color and owner filters
    ///
    /// This method allows for dynamic querying of bikes based on their color and owner.
    /// It demonstrates how to build complex, conditional queries using Diesel.
    ///
    /// # Arguments
    ///
    /// * `color_filter` - An optional filter for the bike's color
    /// * `owner_filter` - An optional filter for the bike's owner
    ///
    /// # Returns
    ///
    /// A vector of bikes matching the filters or a database error
    ///
    /// # How it works
    ///
    /// 1. It starts with a base query for all bikes.
    /// 2. If a color filter is provided, it modifies the query based on the filter type:
    ///    - Equal: Matches bikes with the exact color ID
    ///    - NotEqual: Matches bikes with a different color ID
    ///    - Like: Matches bikes where the color ID is similar (useful for partial matches)
    ///    - In: Matches bikes where the color ID is in a list of values
    /// 3. If an owner filter is provided, it applies similar logic to the owner ID.
    /// 4. Finally, it executes the built query and returns the results.
    ///
    /// This approach allows for flexible and powerful querying without the need for
    /// multiple specific methods for each combination of filters.
    pub fn find_with_filters(&self, color_filter: Option<StringFilter>, owner_filter: Option<StringFilter>) -> QueryResult<Vec<Bike>> {
        let mut conn = self.pool.get().expect("Couldn't get DB connection");
        let mut query = bike.into_boxed();

        if let Some(cf) = color_filter {
            query = match cf {
                StringFilter::Equal(val) => query.filter(color_id.eq(val)),
                StringFilter::NotEqual(val) => query.filter(color_id.ne(val)),
                StringFilter::Like(val) => query.filter(color_id.like(val)),
                StringFilter::In(vals) => query.filter(color_id.eq_any(vals)),
            };
        }

        if let Some(of) = owner_filter {
            query = match of {
                StringFilter::Equal(val) => query.filter(owner_id.eq(val)),
                StringFilter::NotEqual(val) => query.filter(owner_id.ne(val)),
                StringFilter::Like(val) => query.filter(owner_id.like(val)),
                StringFilter::In(vals) => query.filter(owner_id.eq_any(vals)),
            };
        }

        query.load::<Bike>(&mut conn)
    }
}
