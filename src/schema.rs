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
    locations (id) {
        id -> Integer,
        user_id -> Text,
        geo_location -> Integer,
        home_location -> Integer,
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
    user_junctions (id) {
        id -> Integer,
        user_id -> Text,
        inventory_id -> Integer,
        location_id -> Integer,
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
joinable!(locations -> users (user_id));
joinable!(profiles -> users (id));
joinable!(user_junctions -> inventories (inventory_id));
joinable!(user_junctions -> locations (location_id));
joinable!(user_junctions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    inventories,
    items,
    locations,
    profiles,
    user_junctions,
    users,
);
