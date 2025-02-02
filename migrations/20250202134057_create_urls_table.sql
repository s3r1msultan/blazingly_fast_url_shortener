CREATE TABLE IF NOT EXISTS urls (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    original_url TEXT NOT NULL,
    short_url TEXT UNIQUE NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    expiration_date DATETIME NOT NULL DEFAULT (DATETIME(CURRENT_TIMESTAMP, '+1 year'))
);


INSERT INTO urls (original_url, short_url) VALUES ( 'https://github.com/s3r1msultan', 's3r1msultan');
INSERT INTO urls (original_url, short_url) VALUES ('https://blazinglyfast.net', 'blazinglyfast');
INSERT INTO urls (original_url, short_url) VALUES ('https://youtube.com', 'youtube');
INSERT INTO urls (original_url, short_url) VALUES ('https://google.com', '5b1a2675'); -- CTC32