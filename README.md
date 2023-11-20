# omelhorsite-accounts
 Accounts service from omelhorsite.pt

## Routes

#### `GET /`

- **Description**: Root endpoint.
- **Returns**: A string with the message: "Deus quer, o homem sonha, a obra nasce."

#### `POST /create`

- **Description**: Create a new account.
- **Request Payload**: JSON body with the following properties:
  - `handle` (String): Account handle.
  - `name` (String): Account name.
  - `email` (String): Account email.
  - `password` (String): Account password.
  - `gender` (String): Account gender (`male`, `female`, or `not_specified`).
  - `language` (String): Account language.
- **Validation Rules**:
  - `handle`: Minimum length of 1 character, maximum of the specified number in the environment.
  - `name`: Minimum length of 1 character, maximum of the specified number in the environment.
  - `email`: Must be a valid email address.
  - `password`: Minimum length of 1 character.
  - `gender`: One of: `male`, `female`, or `not_specified`.
  - `language`: Minimum length of 1 character.
- **Returns**:
  - Status Code `201 Created` if the account is created successfully.
  - Status Code `422 Unprocessable Entity` if the request payload is invalid.
  - Status Code `409 Conflict` if there is a unique constraint violation, along with the relevant constraint part in the response body. (ex. `handle`, `email`...)
