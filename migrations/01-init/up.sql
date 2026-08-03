CREATE TABLE user (
  id TEXT PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL,
  scrobblingEnabled BOOLEAN DEFAULT 0,
  maxBitRate NUM,
  adminRole BOOLEAN DEFAULT 0,
  settingsRole BOOLEAN DEFAULT 1,
  downloadRole BOOLEAN DEFAULT 0,
  uploadRole BOOLEAN DEFAULT 0,
  playlistRole BOOLEAN DEFAULT 1,
  coverArtRole BOOLEAN DEFAULT 1,
  commentRole BOOLEAN DEFAULT 0,
  podcastRole BOOLEAN DEFAULT 0,
  streamRole BOOLEAN DEFAULT 1,
  jukeboxRole BOOLEAN DEFAULT 0,
  videoConversionRole BOOLEAN DEFAULT 0,
  avatarLastChanged TEXT,
  created TEXT NOT NULL,
  accessed TEXT NOT NULL
);

CREATE TABLE client_keys (
  FOREIGN KEY (userId) REFERENCES user(id),
  key TEXT NOT NULL UNIQUE,
  created TEXT NOT NULL,
  accessed TEXT
);
