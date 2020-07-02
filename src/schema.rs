table! {
    inventories (id) {
        id -> Integer,
        user_id -> Text,
        total_items -> Integer,
    }
}

table! {
    items (id) {
        id -> Integer,
        inventory_id -> Integer,
        dimensions -> Integer,
        weight -> Integer,
        value -> Integer,
        description -> Text,
    }
}

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

joinable!(inventories -> users (user_id));
joinable!(items -> inventories (inventory_id));
joinable!(profiles -> users (id));

allow_tables_to_appear_in_same_query!(
    inventories,
    items,
    profiles,
    users,
);
