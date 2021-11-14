table! {
    users (username) {
        username -> Text,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        locked -> Bool,
    }
}
