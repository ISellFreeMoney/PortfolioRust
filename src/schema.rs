// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Nullable<Text>,
        title -> Text,
        description -> Nullable<Text>,
        completed -> Bool,
    }
}
