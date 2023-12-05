# omelhorsite-accounts

This document provides a brief overview of the API endpoints available in the application.

## Root

- **Endpoint:** `/`
- **Method:** `GET`
- **Response:** A string message.

## Account Creation

### Begin Account Creation

- **Endpoint:** `/create/begin`
- **Method:** `POST`
- **Request Body:**
  - `handle`: String (required)
  - `email`: String (required)
- **Response:**
  - `200 OK` if the account creation process has begun successfully.
  - `422 Unprocessable Entity` if the request body is invalid.
  - `409 Conflict` if the email or handle is already in use.
  - `500 Internal Server Error` for any other errors.

### Finish Account Creation

- **Endpoint:** `/create/finish`
- **Method:** `POST`
- **Request Body:**
  - `verification_code`: String (required)
  - `handle`: String (required)
  - `name`: String (required)
  - `email`: String (required)
  - `password`: String (required)
  - `gender`: Enum (required) - Options: `male`, `female`, `not_specified`
  - `theme`: Enum (required) - Options: `dark`, `light`, `automatic`
  - `language`: String (required)
- **Response:**
  - `200 OK` if the account creation process has finished successfully.
  - `422 Unprocessable Entity` if the request body is invalid.
  - `404 Not Found` if the verification code for the given email and handle does not exist.
  - `500 Internal Server Error` for any other errors.
