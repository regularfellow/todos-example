# Connected TODOs Example

## Setup

1. Declare the database URL

   ```
   export DATABASE_URL="postgres://postgres:password@localhost/connected_todos"
   ```

2. Create the database.

   ```
   $ sqlx db create
   ```

3. Run sql migrations

   ```
   $ sqlx migrate run
   ```

## Usage

Check for errors

```
cargo check
```
