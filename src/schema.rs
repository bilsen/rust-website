table! {
    problems (id) {
        id -> Int4,
        data -> Jsonb,
    }
}

table! {
    rating_history (id) {
        id -> Int4,
        user_id -> Int4,
        time -> Timestamp,
    }
}

table! {
    registrations (id) {
        id -> Int4,
        user_id -> Int4,
        time -> Timestamp,
    }
}

table! {
    submissions (id) {
        id -> Int4,
        problem_id -> Int4,
        user_id -> Int4,
        time -> Timestamp,
        content -> Jsonb,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email_adress -> Text,
        password_hash -> Text,
        rating -> Int4,
        preferences -> Jsonb,
    }
}

allow_tables_to_appear_in_same_query!(
    problems,
    rating_history,
    registrations,
    submissions,
    users,
);
