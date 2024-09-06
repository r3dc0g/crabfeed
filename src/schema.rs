// @generated automatically by Diesel CLI.

diesel::table! {
    author (id) {
        id -> Integer,
        name -> Text,
        uri -> Nullable<Text>,
        email -> Nullable<Text>,
    }
}

diesel::table! {
    category (id) {
        id -> Integer,
        term -> Text,
        scheme -> Nullable<Text>,
        label -> Nullable<Text>,
    }
}

diesel::table! {
    content (id) {
        id -> Integer,
        body -> Nullable<Text>,
        content_type -> Nullable<Text>,
        length -> Nullable<BigInt>,
        src -> Nullable<Integer>,
    }
}

diesel::table! {
    entry (id) {
        id -> Integer,
        feed_id -> Integer,
        title -> Nullable<Text>,
        updated -> Nullable<Timestamp>,
        content_id -> Nullable<Integer>,
        media_id -> Nullable<Integer>,
        summary -> Nullable<Text>,
        source -> Nullable<Text>,
        read -> Nullable<Bool>,
    }
}

diesel::table! {
    entry_author (id) {
        id -> Integer,
        author_id -> Integer,
        entry_id -> Integer,
    }
}

diesel::table! {
    entry_category (id) {
        id -> Integer,
        category_id -> Integer,
        entry_id -> Integer,
    }
}

diesel::table! {
    entry_link (id) {
        id -> Integer,
        link_id -> Integer,
        entry_id -> Integer,
    }
}

diesel::table! {
    feed (id) {
        id -> Integer,
        title -> Nullable<Text>,
        updated -> Nullable<Timestamp>,
        description -> Nullable<Text>,
        language -> Nullable<Text>,
        published -> Nullable<Timestamp>,
    }
}

diesel::table! {
    feed_author (id) {
        id -> Integer,
        author_id -> Integer,
        feed_id -> Integer,
    }
}

diesel::table! {
    feed_category (id) {
        id -> Integer,
        category_id -> Integer,
        feed_id -> Integer,
    }
}

diesel::table! {
    feed_link (id) {
        id -> Integer,
        link_id -> Integer,
        feed_id -> Integer,
    }
}

diesel::table! {
    link (id) {
        id -> Integer,
        href -> Text,
        rel -> Nullable<Text>,
        media_type -> Nullable<Text>,
        href_lang -> Nullable<Text>,
        title -> Nullable<Text>,
        length -> Nullable<BigInt>,
    }
}

diesel::table! {
    media (id) {
        id -> Integer,
        title -> Nullable<Text>,
        thumbnail -> Nullable<Text>,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    media_link (id) {
        id -> Integer,
        link_id -> Integer,
        media_id -> Integer,
    }
}


diesel::joinable!(entry -> feed (feed_id));
diesel::joinable!(entry_author -> author (author_id));
diesel::joinable!(entry_author -> entry (entry_id));
diesel::joinable!(entry_category -> category (category_id));
diesel::joinable!(entry_category -> entry (entry_id));
diesel::joinable!(entry_link -> entry (entry_id));
diesel::joinable!(entry_link -> link (link_id));
diesel::joinable!(feed_author -> author (author_id));
diesel::joinable!(feed_author -> feed (feed_id));
diesel::joinable!(feed_category -> category (category_id));
diesel::joinable!(feed_category -> feed (feed_id));
diesel::joinable!(feed_link -> feed (feed_id));
diesel::joinable!(feed_link -> link (link_id));
diesel::joinable!(media_link -> link (link_id));
diesel::joinable!(media_link -> media (media_id));

diesel::allow_tables_to_appear_in_same_query!(
    author,
    category,
    content,
    entry,
    entry_author,
    entry_category,
    entry_link,
    feed,
    feed_author,
    feed_category,
    feed_link,
    link,
);
