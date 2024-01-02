// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Text,
        name -> Text,
        due_date -> Date,
        due_time -> Nullable<Time>,
        complete -> Bool,
    }
}
