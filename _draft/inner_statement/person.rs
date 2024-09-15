use diesel::{
    helper_types::IntoBoxed,
    prelude::*,
    sql_types::{Bool, Nullable},
    sqlite::Sqlite,
};

use super::*;
use crate::*;

table! {
    person (id) {
        id -> Text,
        name -> Text,
    }
}

#[allow(non_camel_case_types)]
pub(super) enum Condition {
    name(StringFilter),
    bike(Vec<bike::Condition>),
    And(Vec<Condition>),
    Or(Vec<Condition>),
}

type ConditionSource = person::dsl::person;
// Need this type for common condition expressions
type BoxedCondition = Box<dyn BoxableExpression<ConditionSource, Sqlite, SqlType = Nullable<Bool>>>;

type QuerySource = person::dsl::person;
type BoxedQuery = IntoBoxed<'static, QuerySource, Sqlite>;

impl Condition {
    fn to_boxed_condition(self) -> Option<BoxedCondition> {
        Some(match self {
            Condition::name(f) => string_filter!(f, person::dsl::name),
            Condition::And(conditions) => match create_filter(conditions, AndOr::And) {
                Some(boxed_condition) => boxed_condition,
                None => return None,
            },
            Condition::Or(conditions) => match create_filter(conditions, AndOr::Or) {
                Some(boxed_condition) => boxed_condition,
                None => return None,
            },
            Condition::bike(conditions) => {
                // Inner statement, reusing conditions defined in bike
                let inner_statement = bike::create_filtered_query(conditions);
                Box::new(
                    person::dsl::id
                        .eq_any(inner_statement.select(bike::bike::dsl::owner_id))
                        .nullable(),
                )
            }
        })
    }
}

// This method can also be made into a macro, but it should be fine to just duplicate
fn create_filter(conditions: Vec<Condition>, and_or: AndOr) -> Option<BoxedCondition> {
    conditions
        .into_iter()
        // Map into array of boxed conditions
        .filter_map::<BoxedCondition, _>(Condition::to_boxed_condition)
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

pub(super) fn create_filtered_query(conditions: Vec<Condition>) -> BoxedQuery {
    let boxed_query = person::dsl::person.into_boxed();

    match create_filter(conditions, AndOr::And) {
        Some(boxed_conditions) => boxed_query.filter(boxed_conditions),
        None => boxed_query,
    }
}
