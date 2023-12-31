// @generated automatically by Diesel CLI.

diesel::table! {
    author (author_id) {
        author_id -> Integer,
        name -> Text,
        uri -> Nullable<Text>,
        email -> Nullable<Text>,
    }
}

diesel::table! {
    category (category_id) {
        category_id -> Integer,
        term -> Text,
        scheme -> Nullable<Text>,
        label -> Nullable<Text>,
    }
}

diesel::table! {
    content (content_id) {
        content_id -> Integer,
        body -> Nullable<Text>,
        content_type -> Nullable<Text>,
        length -> Nullable<BigInt>,
        src -> Nullable<Integer>,
    }
}

diesel::table! {
    entry (entry_id) {
        entry_id -> Integer,
        feed_id -> Integer,
        title -> Nullable<Text>,
        updated -> Nullable<Timestamp>,
        content_id -> Nullable<Integer>,
        summary -> Nullable<Text>,
        source -> Nullable<Text>,
    }
}

diesel::table! {
    entry_author (author_id, entry_id) {
        author_id -> Integer,
        entry_id -> Integer,
    }
}

diesel::table! {
    entry_category (category_id, entry_id) {
        category_id -> Integer,
        entry_id -> Integer,
    }
}

diesel::table! {
    entry_link (link_id, entry_id) {
        link_id -> Integer,
        entry_id -> Integer,
    }
}

diesel::table! {
    feed (feed_id) {
        feed_id -> Integer,
        title -> Nullable<Text>,
        updated -> Nullable<Timestamp>,
        description -> Nullable<Text>,
        language -> Nullable<Text>,
        published -> Nullable<Timestamp>,
    }
}

diesel::table! {
    feed_author (author_id, feed_id) {
        author_id -> Integer,
        feed_id -> Integer,
    }
}

diesel::table! {
    feed_category (category_id, feed_id) {
        category_id -> Integer,
        feed_id -> Integer,
    }
}

diesel::table! {
    feed_link (link_id, feed_id) {
        link_id -> Integer,
        feed_id -> Integer,
    }
}

diesel::table! {
    link (link_id) {
        link_id -> Integer,
        href -> Text,
        rel -> Nullable<Text>,
        media_type -> Nullable<Text>,
        href_lang -> Nullable<Text>,
        title -> Nullable<Text>,
        length -> Nullable<BigInt>,
    }
}

diesel::joinable!(content -> link (src));
diesel::joinable!(entry -> content (content_id));
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
