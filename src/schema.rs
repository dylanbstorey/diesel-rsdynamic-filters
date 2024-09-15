// @generated automatically by Diesel CLI.

diesel::table! {
    bike (id) {
        id -> Text,
        name -> Text,
        owner_id -> Nullable<Text>,
        color_id -> Nullable<Text>,
    }
}

diesel::table! {
    bike_trip (id) {
        id -> Text,
        name -> Text,
        bike_id -> Nullable<Text>,
    }
}

diesel::table! {
    color (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::table! {
    person (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::joinable!(bike -> color (color_id));
diesel::joinable!(bike -> person (owner_id));
diesel::joinable!(bike_trip -> bike (bike_id));

diesel::allow_tables_to_appear_in_same_query!(
    bike,
    bike_trip,
    color,
    person,
);
