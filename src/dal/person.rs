use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::pg::Pg;
use diesel::sql_types::{Bool, Nullable};
use diesel::helper_types::IntoBoxed;
use crate::models::person::{Person, NewPerson, PersonCondition};
use crate::schema;
use crate::models::common::StringFilter;
use crate::models::AndOr;
use crate::dal::string_filter;
use crate::dal::bike;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

type ConditionSource = schema::person::dsl::person;
type BoxedCondition = Box<dyn BoxableExpression<ConditionSource, Pg, SqlType = Nullable<Bool>>>;
type QuerySource = schema::person::dsl::person;
type BoxedQuery = IntoBoxed<'static, QuerySource, Pg>;

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
        diesel::insert_into(schema::person::table)
            .values(new_person)
            .get_result(&mut conn)
    }

    // Read (by id)
    pub fn find_by_id(&self, person_id: &str) -> QueryResult<Person> {
        let mut conn = self.pool.get().expect("Couldn't get DB connection");
        schema::person::table.find(person_id).first(&mut conn)
    }

    // Read (all)
    pub fn find_all(&self) -> QueryResult<Vec<Person>> {
        let mut conn = self.pool.get().expect("Couldn't get DB connection");
        schema::person::table.load::<Person>(&mut conn)
    }

    // Update
    pub fn update(&self, person_id: &str, updated_person: &Person) -> QueryResult<Person> {
        let mut conn = self.pool.get().expect("Couldn't get DB connection");
        diesel::update(schema::person::table.find(person_id))
            .set(schema::person::name.eq(&updated_person.name))
            .get_result(&mut conn)
    }

    // Delete
    pub fn delete(&self, person_id: &str) -> QueryResult<usize> {
        let mut conn = self.pool.get().expect("Couldn't get DB connection");
        diesel::delete(schema::person::table.find(person_id)).execute(&mut conn)
    }

    // Find with filters
    pub fn find_with_filters(&self, conditions: Vec<PersonCondition>) -> QueryResult<Vec<Person>> {
        let mut conn = self.pool.get().expect("Couldn't get DB connection");
        
        let query = create_filtered_query(conditions);

        query.load::<Person>(&mut conn)
    }
}

impl PersonCondition {
    fn to_boxed_condition(self) -> Option<BoxedCondition> {
        Some(match self {
            PersonCondition::name(f) => string_filter!(f, schema::person::dsl::name),
            PersonCondition::And(conditions) => match create_filter(conditions, AndOr::And) {
                Some(boxed_condition) => boxed_condition,
                None => return None,
            },
            PersonCondition::Or(conditions) => match create_filter(conditions, AndOr::Or) {
                Some(boxed_condition) => boxed_condition,
                None => return None,
            },
            PersonCondition::bike(conditions) => {
                // Inner statement, reusing conditions defined in bike
                let inner_statement = bike::create_filtered_query(conditions);
                Box::new(
                    schema::person::dsl::id
                        .eq_any(inner_statement.select(schema::bike::dsl::owner_id).into_boxed())
                        .nullable(),
                )
            }
        })
    }
}

fn create_filter(conditions: Vec<PersonCondition>, and_or: AndOr) -> Option<BoxedCondition> {
    conditions
        .into_iter()
        .filter_map::<BoxedCondition, _>(PersonCondition::to_boxed_condition)
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

fn create_filtered_query(conditions: Vec<PersonCondition>) -> BoxedQuery {
    let boxed_query = schema::person::table.into_boxed();

    match create_filter(conditions, AndOr::And) {
        Some(boxed_conditions) => boxed_query.filter(boxed_conditions),
        None => boxed_query,
    }
}