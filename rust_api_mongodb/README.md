# rust_api_mongodb

A sample REST API for a **dog walking** booking system. It is built with [Actix Web](https://actix.rs/) and stores data in [MongoDB](https://www.mongodb.com/).

The API lets you register owners, register dogs, create walk bookings, list active bookings (with owner and dogs joined in), and cancel a booking.

## What you need

- Rust (stable toolchain; this crate uses edition `2024`)
- A running MongoDB instance (local default is `mongodb://localhost:27017`)

## How to run

From this directory (`rust_api_mongodb`):

```bash
cargo run
```

The HTTP server binds to **`localhost:5001`**.

MongoDB connection uses the `MONGO_URI` environment variable. If it is not set, the API falls back to:

```text
mongodb://localhost:27017/?directConnection=true
```

On Windows PowerShell:

```powershell
$env:MONGO_URI = "mongodb://localhost:27017/?directConnection=true"
cargo run
```

The database name is **`dog_walking`**. Collections are created on first write:

| Collection | Purpose |
|------------|---------|
| `owner`    | Dog owners |
| `dog`      | Dogs, each linked to an owner |
| `booking`  | Walk bookings |

There is a `src/.env` file with example values (`SERVER`, `PORT`, `MONGO_URI`). **Those values are not loaded automatically** — the crate does not use a dotenv library. Only `MONGO_URI` is read from the process environment. Host and port are currently hardcoded in `src/main.rs`.

## Domain model

```text
Owner 1───* Dog
Owner 1───* Booking
```

- An **owner** has name, email, phone, and address.
- A **dog** belongs to one owner (`owner` is a MongoDB `ObjectId`). Name, age, and breed are optional.
- A **booking** belongs to one owner, has a start time, duration in minutes, and a `cancelled` flag. Cancelled bookings are excluded from the list endpoint.

`GET /bookings` returns a **full booking**: the booking fields plus the nested **owner** document and that owner’s **dogs**.

## Request flow (what actually runs)

```text
HTTP request
    → routes (src/routes)
    → handlers (src/handlers)
    → services (src/services)
    → Database methods on config::db::Database (src/config/db.rs)
    → MongoDB
```

`main` creates one `Database` instance, wraps it in Actix `web::Data`, and registers the booking, dog, and owner route configs.

## Project layout

```text
src/
├── main.rs                 # App startup, Mongo init, bind localhost:5001
├── config/
│   ├── db.rs               # Live Database type + CRUD used by services
│   └── database.rs         # Slimmer Database (init + collections only)
├── models/                 # Documents and JSON request bodies
├── handlers/               # HTTP handlers (status codes + JSON)
├── routes/                 # Maps URLs to handlers
├── services/               # Request → model conversion, then DB calls
└── repositories/           # Trait-based data access (not wired into the app yet)
    └── db_traits/
```

| Layer | Role |
|-------|------|
| **Models** | `Owner` / `Dog` / `Booking` documents and `*Request` DTOs. `TryFrom` turns a request into a document (new `ObjectId`, parsed owner id, RFC 3339 start time). |
| **Handlers** | Parse JSON / path params, call a service, return `200` or `500`. |
| **Services** | Build domain structs and call `Database`. |
| **Config `db.rs`** | Connection + collection access + the Mongo operations the API uses today. |
| **Repositories** | Intended split: traits (`OwnerDbOperation`, `DogDbOperation`, `BookingDbOperation`) and Mongo implementations. Handlers and services still call `Database` directly. |

`src/handlers/health_check_handler.rs` defines `GET /health`, but that route is **not** registered in `main` or any `routes` module, so it is not served.

## API

Base URL: `http://localhost:5001`

Successful creates return MongoDB’s insert result (includes `inserted_id`). Errors currently return **500** with a string body.

### Create owner

`POST /owner`

```json
{
  "name": "Ada Lovelace",
  "email": "ada@example.com",
  "phone": "555-0100",
  "address": "12 Analytical Engine Rd"
}
```

### Create dog

`POST /dog`

`owner` must be a valid 24-character MongoDB ObjectId string from a created owner.

```json
{
  "owner": "507f1f77bcf86cd799439011",
  "name": "Nala",
  "age": 4,
  "breed": "Labrador"
}
```

`name`, `age`, and `breed` may be omitted (`null`).

### Create booking

`POST /booking`

`start_time` must be RFC 3339 (for example `2026-09-20T15:00:00Z`).

```json
{
  "owner": "507f1f77bcf86cd799439011",
  "start_time": "2026-09-20T15:00:00Z",
  "duration_in_minutes": 30
}
```

New bookings are stored with `cancelled: false`.

### List active bookings

`GET /bookings`

Returns bookings where `cancelled` is not `true`, each joined with the owner and that owner’s dogs (`$lookup` aggregation in `get_bookings`).

### Cancel booking

`PUT /booking/{id}/cancel`

`id` is the booking’s ObjectId. Sets `cancelled` to `true`.

## Typical usage order

1. Create an owner and copy `inserted_id`.
2. Create one or more dogs with that owner id.
3. Create a booking with the same owner id.
4. Call `GET /bookings` to see owner + dogs on each active booking.
5. Cancel with `PUT /booking/{id}/cancel`.

Example with curl:

```bash
curl -X POST http://localhost:5001/owner \
  -H "Content-Type: application/json" \
  -d "{\"name\":\"Ada Lovelace\",\"email\":\"ada@example.com\",\"phone\":\"555-0100\",\"address\":\"12 Analytical Engine Rd\"}"

curl http://localhost:5001/bookings
```

## Dependencies

Defined in `Cargo.toml`:

- `actix-web` — HTTP server and routing
- `mongodb` — driver (v3)
- `serde` — JSON / BSON serialization
- `chrono` — parse booking `start_time`
- `futures-util` — stream the aggregation cursor
- `async-trait` — async traits on the repository layer

## Notes while reading the code

- **Two `Database` structs:** `config::db::Database` is what the running app uses. `config::database::Database` is the collection holder used by some repository files.
- **Repositories are incomplete:** they are not injected in `main`, and several `#[async_trait]` / constructor details would need cleanup before they can replace `db.rs`.
- **IDs in JSON:** Mongo `ObjectId` values serialize as BSON objects unless you add a custom serializer; clients should treat `inserted_id` from create responses as the id to pass in later `owner` / path fields.
