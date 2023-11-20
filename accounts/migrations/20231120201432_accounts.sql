CREATE TABLE accounts (
    "id" TEXT NOT NULL UNIQUE,
    "picture_id" TEXT NOT NULL,
    "handle" TEXT NOT NULL UNIQUE,
    "name" TEXT NOT NULL,
    "email" TEXT NOT NULL UNIQUE,
    "password" TEXT NOT NULL,
    "group" TEXT NOT NULL,
    "gender" TEXT NOT NULL,
    "language" TEXT NOT NULL
)