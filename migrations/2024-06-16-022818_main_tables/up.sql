CREATE TABLE feed (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  title VARCHAR,
  updated DATETIME,
  description TEXT,
  language VARCHAR,
  published DATETIME
);

CREATE TABLE entry (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  feed_id INTEGER UNSIGNED NOT NULL,
  title VARCHAR,
  updated DATETIME,
  content_id INTEGER,
  summary TEXT,
  source VARCHAR,
  read BOOLEAN DEFAULT FALSE,
  FOREIGN KEY(feed_id) REFERENCES feed(feed_id),
  FOREIGN KEY(content_id) REFERENCES content(content_id)
);

CREATE TABLE author (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  name VARCHAR NOT NULL,
  uri VARCHAR,
  email VARCHAR
);

CREATE TABLE link (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  href VARCHAR NOT NULL,
  rel VARCHAR,
  media_type VARCHAR,
  href_lang VARCHAR,
  title VARCHAR,
  length BIGINT
);

CREATE TABLE content (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  body TEXT,
  content_type VARCHAR,
  length BIGINT,
  src INTEGER,
  FOREIGN KEY(src) REFERENCES link(link_id)
);


CREATE TABLE category (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  term VARCHAR NOT NULL,
  scheme VARCHAR,
  label VARCHAR
);

