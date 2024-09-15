use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use crate::models::bike_trip::{BikeTrip, NewBikeTrip};
use crate::schema::bike_trip::dsl::*;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct BikeTripDAL {
    pool: Pool,
}

impl BikeTripDAL {
    pub fn new(pool: Pool) -> Self {
        BikeTripDAL { pool }
    }

    pub fn create(&self, new_bike_trip: &NewBikeTrip) -> QueryResult<BikeTrip> {
        let mut conn = self.pool.get().expect("Couldn't get DB connection");
        diesel::insert_into(bike_trip)
            .values(new_bike_trip)
            .get_result(&mut conn)
    }

    pub fn find_by_id(&self, bike_trip_id: &str) -> QueryResult<BikeTrip> {
        let mut conn = self.pool.get().expect("Couldn't get DB connection");
        bike_trip.find(bike_trip_id).first(&mut conn)
    }

    pub fn find_all(&self) -> QueryResult<Vec<BikeTrip>> {
        let mut conn = self.pool.get().expect("Couldn't get DB connection");
        bike_trip.load::<BikeTrip>(&mut conn)
    }

    pub fn update(&self, bike_trip_id: &str, updated_bike_trip: &BikeTrip) -> QueryResult<BikeTrip> {
        let mut conn = self.pool.get().expect("Couldn't get DB connection");
        diesel::update(bike_trip.find(bike_trip_id))
            .set((
                name.eq(&updated_bike_trip.name),
                bike_id.eq(&updated_bike_trip.bike_id)
            ))
            .get_result(&mut conn)
    }

    pub fn delete(&self, bike_trip_id: &str) -> QueryResult<usize> {
        let mut conn = self.pool.get().expect("Couldn't get DB connection");
        diesel::delete(bike_trip.find(bike_trip_id))
            .execute(&mut conn)
    }
}