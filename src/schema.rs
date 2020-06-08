table! {
    profiles (profileid) {
        profileid -> Integer,
        bio -> Text,
    }
}

table! {
    users (id) {
        id -> Text,
        username -> Text,
        password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    profiles,
    users,
);
