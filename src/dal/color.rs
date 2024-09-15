use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use crate::models::color::{Color, NewColor};
use crate::schema::color::dsl::*;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct ColorDAL {
    pool: Pool,
}

impl ColorDAL {
    pub fn new(pool: Pool) -> Self {
        ColorDAL { pool }
    }

    pub fn create(&self, new_color: &NewColor) -> QueryResult<Color> {
        let mut conn = self.pool.get().expect("Couldn't get DB connection");
        diesel::insert_into(color)
            .values(new_color)
            .get_result(&mut conn)
    }

    pub fn find_by_id(&self, color_id: &str) -> QueryResult<Color> {
        let mut conn = self.pool.get().expect("Couldn't get DB connection");
        color.find(color_id).first(&mut conn)
    }

    pub fn find_all(&self) -> QueryResult<Vec<Color>> {
        let mut conn = self.pool.get().expect("Couldn't get DB connection");
        color.load::<Color>(&mut conn)
    }

    pub fn update(&self, color_id: &str, updated_color: &Color) -> QueryResult<Color> {
        let mut conn = self.pool.get().expect("Couldn't get DB connection");
        diesel::update(color.find(color_id))
            .set(name.eq(&updated_color.name))
            .get_result(&mut conn)
    }

    pub fn delete(&self, color_id: &str) -> QueryResult<usize> {
        let mut conn = self.pool.get().expect("Couldn't get DB connection");
        diesel::delete(color.find(color_id))
            .execute(&mut conn)
    }
}