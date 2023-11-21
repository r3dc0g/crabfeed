-- Your SQL goes here
CREATE TABLE feed (
  feed_id INTEGER UNSIGNED PRIMARY KEY,
  title VARCHAR,
  updated DATETIME,
  description TEXT,
  language VARCHAR,
  published DATETIME,
  rating VARCHAR,
  rights TEXT,
  ttl INTEGER UNSIGNED
);

CREATE TABLE entry (
  entry_id INTEGER UNSIGNED PRIMARY KEY,
  feed_id INTEGER UNSIGNED NOT NULL,
  title VARCHAR,
  updated DATETIME,
  content_id INTEGER,
  summary TEXT,
  source VARCHAR,
  rights TEXT,
  FOREIGN KEY(feed_id) REFERENCES feed(feed_id),
  FOREIGN KEY(content_id) REFERENCES content(content_id)
);

CREATE TABLE author (
  author_id INTEGER UNSIGNED PRIMARY KEY,
  name VARCHAR NOT NULL,
  uri VARCHAR,
  email VARCHAR
);

CREATE TABLE link (
  link_id INTEGER UNSIGNED PRIMARY KEY,
  href VARCHAR NOT NULL,
  rel VARCHAR,
  media_type VARCHAR,
  href_lang VARCHAR,
  title VARCHAR,
  length INTEGER
);

CREATE TABLE content (
  content_id INTEGER UNSIGNED PRIMARY KEY,
  body TEXT,
  content_type VARCHAR,
  length INTEGER UNSIGNED,
  src INTEGER UNSIGNED,
  FOREIGN KEY(src) REFERENCES link(link_id)
);


CREATE TABLE category (
  category_id INTEGER UNSIGNED PRIMARY KEY,
  term VARCHAR NOT NULL,
  scheme VARCHAR,
  label VARCHAR
);

CREATE TABLE feed_author (
  author_id INTEGER UNSIGNED NOT NULL,
  feed_id INTEGER UNSIGNED NOT NULL,
  FOREIGN KEY(author_id) REFERENCES author(author_id),
  FOREIGN KEY(feed_id) REFERENCES feed(feed_id),
  PRIMARY KEY(author_id, feed_id)
);

CREATE TABLE entry_author (
  author_id INTEGER UNSIGNED NOT NULL,
  entry_id INTEGER UNSIGNED NOT NULL,
  FOREIGN KEY(author_id) REFERENCES author(author_id),
  FOREIGN KEY(entry_id) REFERENCES entry(entry_id),
  PRIMARY KEY(author_id, entry_id)
);

CREATE TABLE feed_link (
  link_id INTEGER UNSIGNED NOT NULL,
  feed_id INTEGER UNSIGNED NOT NULL,
  FOREIGN KEY(link_id) REFERENCES link(link_id),
  FOREIGN KEY(feed_id) REFERENCES feed(feed_id),
  PRIMARY KEY(link_id, feed_id)
);

CREATE TABLE entry_link (
  link_id INTEGER UNSIGNED NOT NULL,
  entry_id INTEGER UNSIGNED NOT NULL,
  FOREIGN KEY(link_id) REFERENCES link(link_id),
  FOREIGN KEY(entry_id) REFERENCES entry(entry_id),
  PRIMARY KEY(link_id, entry_id)
);

CREATE TABLE feed_category (
  category_id INTEGER UNSIGNED NOT NULL,
  feed_id INTEGER UNSIGNED NOT NULL,
  FOREIGN KEY(category_id) REFERENCES category(category_id),
  FOREIGN KEY(feed_id) REFERENCES feed(feed_id),
  PRIMARY KEY(category_id, feed_id)
);

CREATE TABLE entry_category (
  category_id INTEGER UNSIGNED NOT NULL,
  entry_id INTEGER UNSIGNED NOT NULL,
  FOREIGN KEY(category_id) REFERENCES category(category_id),
  FOREIGN KEY(entry_id) REFERENCES entry(entry_id),
  PRIMARY KEY(category_id, entry_id)
);
