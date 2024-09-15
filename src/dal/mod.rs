use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

mod person;
mod bike;
mod color;
mod bike_trip;


pub use person::PersonDAL;
pub use bike::BikeDAL;
pub use color::ColorDAL;
pub use bike_trip::BikeTripDAL;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct DataAccessLayer {
    pool: Pool,
}

impl DataAccessLayer {
    pub fn new(pool: Pool) -> Self {
        DataAccessLayer { pool }
    }

    pub fn person(&self) -> PersonDAL {
        PersonDAL::new(self.pool.clone())
    }

    pub fn bike(&self) -> BikeDAL {
        BikeDAL::new(self.pool.clone())
    }

    pub fn color(&self) -> ColorDAL {
        ColorDAL::new(self.pool.clone())
    }

    pub fn bike_trip(&self) -> BikeTripDAL {
        BikeTripDAL::new(self.pool.clone())
    }

}


enum NumberFilter<T> {
    Equal(T),
    NotEqual(T),
    GreaterThen(T),
    LowerThen(T),
    IsNull,
    IsNotNull,
}

macro_rules! number_filter {
    ($filter:ident, $dsl_field:expr ) => {{
        match $filter {
            NumberFilter::Equal(value) => Box::new($dsl_field.eq(value).nullable()),
            NumberFilter::NotEqual(value) => Box::new($dsl_field.ne(value).nullable()),
            NumberFilter::GreaterThen(value) => Box::new($dsl_field.gt(value).nullable()),
            NumberFilter::LowerThen(value) => Box::new($dsl_field.lt(value).nullable()),
            NumberFilter::IsNull => Box::new($dsl_field.is_null().nullable()),
            NumberFilter::IsNotNull => Box::new($dsl_field.is_not_null().nullable()),
        }
    }};
}

enum StringFilter {
    Equal(String),
    NotEqual(String),
    Like(String),
    In(Vec<String>),
}

macro_rules! string_filter {
    ($filter:ident, $dsl_field:expr ) => {{
        match $filter {
            StringFilter::Equal(value) => Box::new($dsl_field.eq(value).nullable()),
            StringFilter::NotEqual(value) => Box::new($dsl_field.ne(value).nullable()),
            StringFilter::Like(value) => Box::new($dsl_field.like(value).nullable()),
            StringFilter::In(value) => Box::new($dsl_field.eq_any(value).nullable()),
        }
    }};
}

enum BooleanFilter {
    True,
    False,
    IsNull,
    IsNotNull,
}

macro_rules! boolean_filter {
    ($filter:ident, $dsl_field:expr ) => {{
        match $filter {
            BooleanFilter::True => Box::new($dsl_field.eq(true).nullable()),
            BooleanFilter::False => Box::new($dsl_field.eq(false).nullable()),
            BooleanFilter::IsNull => Box::new($dsl_field.is_null().nullable()),
            BooleanFilter::IsNotNull => Box::new($dsl_field.is_not_null().nullable()),
        }
    }};
}

enum AndOr {
    And,
    Or,
}

use boolean_filter;
use number_filter;
use string_filter;