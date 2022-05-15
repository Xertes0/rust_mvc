diesel::table! {
    products (id) {
        id -> Integer,
        name -> Text,
        image -> Text,
        description -> Text,
        price -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        password -> Text,
        privilege -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    products,
    users,
);
