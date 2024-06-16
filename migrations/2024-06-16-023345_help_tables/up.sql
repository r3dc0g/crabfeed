CREATE TABLE feed_author (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  author_id INTEGER NOT NULL,
  feed_id INTEGER NOT NULL,
  FOREIGN KEY(author_id) REFERENCES author(author_id),
  FOREIGN KEY(feed_id) REFERENCES feed(feed_id)
);

CREATE TABLE entry_author (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  author_id INTEGER NOT NULL,
  entry_id INTEGER NOT NULL,
  FOREIGN KEY(author_id) REFERENCES author(author_id),
  FOREIGN KEY(entry_id) REFERENCES entry(entry_id)
);

CREATE TABLE feed_link (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  link_id INTEGER NOT NULL,
  feed_id INTEGER NOT NULL,
  FOREIGN KEY(link_id) REFERENCES link(link_id),
  FOREIGN KEY(feed_id) REFERENCES feed(feed_id)
);

CREATE TABLE entry_link (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  link_id INTEGER NOT NULL,
  entry_id INTEGER NOT NULL,
  FOREIGN KEY(link_id) REFERENCES link(link_id),
  FOREIGN KEY(entry_id) REFERENCES entry(entry_id)
);

CREATE TABLE feed_category (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  category_id INTEGER NOT NULL,
  feed_id INTEGER NOT NULL,
  FOREIGN KEY(category_id) REFERENCES category(category_id),
  FOREIGN KEY(feed_id) REFERENCES feed(feed_id)
);

CREATE TABLE entry_category (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  category_id INTEGER NOT NULL,
  entry_id INTEGER NOT NULL,
  FOREIGN KEY(category_id) REFERENCES category(category_id),
  FOREIGN KEY(entry_id) REFERENCES entry(entry_id)
);
