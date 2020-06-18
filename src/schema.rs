table! {
    profiles (profile_id) {
        profile_id -> Integer,
        id -> Text,
        bio -> Text,
        age -> Integer,
        community -> Text,
    }
}

table! {
    users (id) {
        id -> Text,
        username -> Text,
        password -> Text,
    }
}

joinable!(profiles -> users (id));

allow_tables_to_appear_in_same_query!(
    profiles,
    users,
);
