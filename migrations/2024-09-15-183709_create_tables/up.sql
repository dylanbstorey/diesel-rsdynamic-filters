-- Your SQL goes here

-- Create person table
CREATE TABLE person (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL
);

-- Create color table
CREATE TABLE color (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL
);

-- Create bike table
CREATE TABLE bike (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    owner_id TEXT REFERENCES person(id),
    color_id TEXT REFERENCES color(id)
);

-- Create bike_trip table
CREATE TABLE bike_trip (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    bike_id TEXT REFERENCES bike(id)
);

