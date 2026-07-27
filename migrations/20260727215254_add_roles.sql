-- Add migration script here
Create type ROLE as ENUM('Admin', 'User');

Alter table users
add role ROLE NOT NULL Default 'User'
