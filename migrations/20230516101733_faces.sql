CREATE TABLE IF NOT EXISTS faces (
                              id INTEGER PRIMARY KEY,
                              uuid VARCHAR NOT NULL,
                              photo_uuid VARCHAR NOT NULL,
                              person_uuid VARCHAR ,
                              measurements TEXT NOT NULL,
                              moderated BOOL DEFAULT false
                            );