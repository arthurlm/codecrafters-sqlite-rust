[![progress-banner](https://backend.codecrafters.io/progress/sqlite/902fb9cd-428c-4d4b-9a50-0d0bc0f1a3d2)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

# Codecrafters Sqlite

Solution to the [Codecrafters Sqlite](https://app.codecrafters.io/courses/sqlite/completed) challenge.

- Everything works.
- Contains comment on code.
- Have minimal usage on `unwrap` / `expect`.
- Code is correctly split into multi simple module with single responsability.
- Use `peg` crate to parse SQL ❤️.
- Use `nom` crate to parse Sqlite binary file 😎.
- Use minimal possible extra crate 😁.

## What can be improved in this code

- Add more unit tests.
- Index full scan can be improved.
- Search by primary key is not supported.
- Some `.clone()` can be easily avoid.
- `SELECT COUNT(*)` implementation is a little bit dirty.
