# task-api

A REST API for managing tasks, built with Axum, Tokio, SQLx, and Serde.

## Stack

- **Axum** — HTTP routing and request handling
- **Tokio** — async runtime
- **SQLx** — async, compile-time-checked SQL toolkit (SQLite in this build)
- **Serde** — JSON serialization/deserialization
- **thiserror** — structured error types

## Structure

- `src/main.rs` — entry point; builds the router and starts the server
- `src/models.rs` — `Task`, `CreateTask`, `UpdateTask` request/response types
- `src/error.rs` — `AppError` enum with an `IntoResponse` impl, mapping each
  error variant to the correct HTTP status code
- `src/db.rs` — connection pool setup and table initialization
- `src/handlers.rs` — one async handler per endpoint

## Running it

```bash
cargo run
```

The server starts on `http://localhost:3000`. It uses an in-memory SQLite
database by default, so data resets on restart. To persist data between
runs, change the connection string in `main.rs` from `"sqlite::memory:"` to
`"sqlite:tasks.db"`.

## API

| Method | Path          | Description                          |
|--------|---------------|---------------------------------------|
| GET    | `/tasks`      | List all tasks                        |
| GET    | `/tasks/:id`  | Get a single task                     |
| POST   | `/tasks`      | Create a task                         |
| PUT    | `/tasks/:id`  | Partially update a task               |
| DELETE | `/tasks/:id`  | Delete a task                         |

### Examples

```bash
# Create a task
curl -X POST localhost:3000/tasks \
  -H "Content-Type: application/json" \
  -d '{"description": "learn axum"}'

# List all tasks
curl localhost:3000/tasks

# Get one task
curl localhost:3000/tasks/1

# Update a task (partial — only send the fields you want to change)
curl -X PUT localhost:3000/tasks/1 \
  -H "Content-Type: application/json" \
  -d '{"done": true}'

# Delete a task
curl -X DELETE localhost:3000/tasks/1
```

Validation failures return `400 Bad Request`, and missing resources return
`404 Not Found`, both with a JSON error body:
```json
{ "error": "task not found" }
```

## Design notes

- **Error handling**: `AppError` is a single enum covering all failure
  modes (`NotFound`, `Validation`, `Database`). `thiserror` generates the
  `Display` implementation from `#[error("...")]` attributes, and
  `#[from]` auto-generates `From<sqlx::Error> for AppError`, so database
  errors convert automatically via `?`. A separate `IntoResponse` impl maps
  each variant to the appropriate HTTP status code, so handlers can just
  return `Result<T, AppError>` and let Axum handle the response.
- **State management**: the SQLite connection pool (`SqlitePool`) is shared
  across all request handlers via Axum's `State` extractor. The pool is
  internally reference-counted, so cloning it (which Axum does per request)
  is cheap and all clones share the same underlying connections.
- **Extractors**: `Path<i64>` and `Json<T>` parse the incoming request into
  typed Rust values directly in the handler signature, so handler bodies
  never touch raw strings or do manual parsing.
