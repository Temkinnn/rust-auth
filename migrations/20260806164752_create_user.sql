Create type ROLE as ENUM('Admin', 'User');

CREATE TABLE IF NOT EXISTS users (
  id UUID PRIMARY KEY DEFAULT uuidv7(),
  username Varchar(20) Unique Not Null,
  email Text Unique Not Null,
  password Text Not Null,
  role ROLE NOT NULL Default 'User',
  created_at Timestamp Default Current_timestamp
);
