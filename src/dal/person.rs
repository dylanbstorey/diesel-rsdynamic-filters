use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use crate::models::person::{Person, NewPerson};
use crate::schema::person::dsl::*;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct PersonDAL {
    pool: Pool,
}

impl PersonDAL {
    pub fn new(pool: Pool) -> Self {
        PersonDAL { pool }
    }

    // Create
    pub fn create(&self, new_person: &NewPerson) -> QueryResult<Person> {
        let mut conn = self.pool.get().expect("Couldn't get DB connection");
        diesel::insert_into(person)
            .values(new_person)
            .get_result(&mut conn)
    }

    // Read (by id)
    pub fn find_by_id(&self, person_id: &str) -> QueryResult<Person> {
        let mut conn = self.pool.get().expect("Couldn't get DB connection");
        person.find(person_id).first(&mut conn)
    }

    // Read (all)
    pub fn find_all(&self) -> QueryResult<Vec<Person>> {
        let mut conn = self.pool.get().expect("Couldn't get DB connection");
        person.load::<Person>(&mut conn)
    }

    // Update
    pub fn update(&self, person_id: &str, updated_person: &Person) -> QueryResult<Person> {
        let mut conn = self.pool.get().expect("Couldn't get DB connection");
        diesel::update(person.find(person_id))
            .set(name.eq(&updated_person.name))
            .get_result(&mut conn)
    }

    // Delete
    pub fn delete(&self, person_id: &str) -> QueryResult<usize> {
        let mut conn = self.pool.get().expect("Couldn't get DB connection");
        diesel::delete(person.find(person_id)).execute(&mut conn)
    }
}