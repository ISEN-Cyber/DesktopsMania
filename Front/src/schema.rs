#[database("principal")]
pub struct DataBase(diesel::PgConnection);

table! {
    level(id)
    {
        id -> Integer,
        value -> Text,
    }
}

table! {
    session (uuid) {
        uuid -> Text,
        path -> Text,
        id_user -> Integer,
        timestamp -> Timestamp,
    }
}

table! {
    temporaire (path) {
        path -> Text,
        id_user -> Integer,
        timestamp -> Timestamp,
        code -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        id_admin -> Integer,
        id_level -> Integer,
        last_name -> Text,
        first_name -> Text,
        email -> Text,
        password -> Nullable<Text>,
        secret -> Nullable<Text>,
        timestamp -> Timestamp,
    }
}


table! {
    vm (id) {
        id -> Integer,
        id_creator -> Integer,
        title -> Text,
        timestamp -> Nullable<Timestamp>,
        link -> Text,
        ip -> Nullable<Text>,
    }
}

table! {
    link_user_vm (id_user,id_vm) {
        id_vm -> Integer,
        id_user -> Integer,
    }
}

table! {
    info (id) {
        id -> Integer,
        id_user -> Integer,
        title -> Text,
        value -> Text,
        start -> Timestamp,
        stop -> Nullable<Timestamp>,
    }
}

table! {
    vemmion (uuid) {
        uuid -> Text,
        timestamp -> Timestamp,
        id_user -> Integer,
        id_vm -> Integer,
        runing -> Bool,
    }
}

joinable!(vm -> users (id_creator));
joinable!(vemmion -> users (id_user));
allow_tables_to_appear_in_same_query!(vemmion, users);
joinable!(vemmion -> vm (id_vm));
allow_tables_to_appear_in_same_query!(vm, vemmion);
joinable!(session -> users (id_user));
allow_tables_to_appear_in_same_query!(session, users);
joinable!(temporaire -> users (id_user));
allow_tables_to_appear_in_same_query!(temporaire, users);
allow_tables_to_appear_in_same_query!(session, level);
joinable!(link_user_vm -> users (id_user));
allow_tables_to_appear_in_same_query!(users, link_user_vm);
joinable!(link_user_vm -> vm (id_vm));
allow_tables_to_appear_in_same_query!(vm, link_user_vm);
allow_tables_to_appear_in_same_query!(vm, users);
joinable!(users -> level (id_level));
allow_tables_to_appear_in_same_query!(level, users);
allow_tables_to_appear_in_same_query!(level, temporaire);
allow_tables_to_appear_in_same_query!(session, temporaire);
allow_tables_to_appear_in_same_query!(session, link_user_vm);
allow_tables_to_appear_in_same_query!(link_user_vm, temporaire);
allow_tables_to_appear_in_same_query!(level, link_user_vm);
