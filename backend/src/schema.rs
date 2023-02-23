// @generated automatically by Diesel CLI.

diesel::table! {
    comments (id) {
        id -> Int4,
        comment -> Varchar,
        user_id -> Int4,
        video_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    sessions (id) {
        id -> Int4,
        sessions_key -> Varchar,
        user_id -> Int4,
        date -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        profile_pic -> Varchar,
    }
}

diesel::table! {
    video (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
        url -> Varchar,
        thumb_mail_url -> Varchar,
        created_at -> Timestamp,
        user_id -> Int4,
    }
}

diesel::table! {
    views (id) {
        id -> Int4,
        video_id -> Int4,
        user_id -> Int4,
    }
}

diesel::joinable!(comments -> users (user_id));
diesel::joinable!(comments -> video (video_id));
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(video -> users (user_id));
diesel::joinable!(views -> users (user_id));
diesel::joinable!(views -> video (video_id));

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    sessions,
    users,
    video,
    views,
);
