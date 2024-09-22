// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "dice"))]
    pub struct Dice;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "tasktype"))]
    pub struct Tasktype;
}

diesel::table! {
    claans (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    records (id) {
        id -> Int4,
        score -> Int4,
        timestamp -> Date,
        task_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    seasons (id) {
        id -> Int4,
        name -> Varchar,
        start_date -> Date,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Tasktype;
    use super::sql_types::Dice;

    tasks (id) {
        id -> Int4,
        description -> Varchar,
        tasktype -> Tasktype,
        dice -> Dice,
        ephemeral -> Bool,
        active -> Bool,
        last -> Nullable<Date>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        claan_id -> Int4,
        #[max_length = 120]
        email -> Varchar,
        password_hash -> Bytea,
        active -> Bool,
        #[max_length = 23]
        current_auth_token -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        last_action -> Nullable<Timestamp>,
    }
}

diesel::joinable!(records -> tasks (task_id));
diesel::joinable!(records -> users (user_id));
diesel::joinable!(users -> claans (claan_id));

diesel::allow_tables_to_appear_in_same_query!(
    claans,
    records,
    seasons,
    tasks,
    users,
);
