// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Text,
        name -> Text,
        due_date -> Text,
        due_time -> Nullable<Text>,
        complete -> Integer,
    }
}
