-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
  id Varchar(100) Primary Key,
  username Varchar(20) Unique Not Null,
  email Text Unique Not Null,
  password Text Not Null,
  created_at Timestamp Default Current_timestamp
);
