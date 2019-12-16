-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL,
    email_adress TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    rating INT NOT NULL,
    preferences JSONB NOT NULL
);

CREATE TABLE registrations (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    time TIMESTAMP NOT NULL
);

CREATE TABLE rating_history (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    time TIMESTAMP NOT NULL
);


CREATE TABLE problems (
    id SERIAL PRIMARY KEY,
    data JSONB NOT NULL
);

CREATE TABLE submissions (
    id SERIAL PRIMARY KEY,
    problem_id INT NOT NULL,
    user_id INT NOT NULL,
    time TIMESTAMP NOT NULL,
    content JSONB NOT NULL
);