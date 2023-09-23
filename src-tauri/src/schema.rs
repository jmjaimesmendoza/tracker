// @generated automatically by Diesel CLI.

diesel::table! {
    equipments (id) {
        id -> Integer,
        name -> Text,
        km -> Integer,
    }
}

diesel::table! {
    logs (id) {
        id -> Integer,
        equipment_id -> Integer,
        person_id -> Integer,
        previous_km -> Integer,
        new_km -> Integer,
        description -> Text,
    }
}

diesel::table! {
    persons (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        hash -> Text,
    }
}

diesel::joinable!(logs -> equipments (equipment_id));
diesel::joinable!(logs -> persons (person_id));

diesel::allow_tables_to_appear_in_same_query!(
    equipments,
    logs,
    persons,
    users,
);
