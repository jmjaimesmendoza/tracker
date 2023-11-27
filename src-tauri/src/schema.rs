// @generated automatically by Diesel CLI.

diesel::table! {
    brands (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    equipments (id) {
        id -> Integer,
        name -> Text,
        km -> Integer,
        model_id -> Integer,
        nserial -> Text,
        notes -> Text,
        file_path -> Text,
    }
}

diesel::table! {
    logs (id) {
        id -> Integer,
        equipment_id -> Integer,
        person_id -> Integer,
        job -> Text,
        km -> Integer,
        description -> Text,
        created_at -> Text,
    }
}

diesel::table! {
    manuals (id) {
        id -> Integer,
        name -> Text,
        file_path -> Text,
        equipment_id -> Integer,
    }
}

diesel::table! {
    models (id) {
        id -> Integer,
        brand_id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    persons (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    revisions (id) {
        id -> Integer,
        equipment_id -> Integer,
        tipo -> Text,
        target -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        hash -> Text,
    }
}

diesel::joinable!(equipments -> models (model_id));
diesel::joinable!(logs -> equipments (equipment_id));
diesel::joinable!(logs -> persons (person_id));
diesel::joinable!(manuals -> equipments (equipment_id));
diesel::joinable!(models -> brands (brand_id));
diesel::joinable!(revisions -> equipments (equipment_id));

diesel::allow_tables_to_appear_in_same_query!(
    brands,
    equipments,
    logs,
    manuals,
    models,
    persons,
    revisions,
    users,
);
