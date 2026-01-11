| name | purpose |
|------|---------|
| create-schema | create a repeatable skill for adding new schemas |

# Create Schema Skill

Creates a new application schema with proper documentation and database migration.

## Usage

```
/create-schema <schema-name>
```

## Process

When creating a new schema, follow these steps:

### 1. Create JSON Schema Documentation

Create a JSON Schema file at `shared/schemas/<schema-name>.json`:

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "<schema-name>.json",
  "title": "<SchemaName>",
  "description": "<Description of the schema>",
  "type": "object",
  "properties": {
    // Define properties here
  },
  "required": []
}
```

### 2. Create Reversible Database Migration

Create two migration files in `backend/migrations/` using the format:
- `<YYYYMMDDHHMMSS>_create_<table_name>.up.sql`
- `<YYYYMMDDHHMMSS>_create_<table_name>.down.sql`

The timestamp should be the current UTC time in format `YYYYMMDDHHMMSS`.

## Required Fields

All schemas **must** include the following fields:

| Field | Type | Description |
|-------|------|-------------|
| `id` | UUIDv4 | Primary key identifier |
| `created_at` | Timestamp | When the record was created |
| `updated_at` | Timestamp | When the record was last modified |

These fields ensure consistent tracking across all entities in the system.

## Type Conventions

### UUID Fields
- **JSON Schema**: `{ "type": "string", "format": "uuid" }`
- **SQLite**: `TEXT PRIMARY KEY NOT NULL` (for primary keys) or `TEXT NOT NULL` (for foreign keys)
- **Rust**: `uuid::Uuid`
- **Generation**: Use `Uuid::new_v4()` for new UUIDs

### Timestamp Fields
- **JSON Schema**: `{ "type": "string", "format": "date-time" }`
- **SQLite**: `TEXT NOT NULL DEFAULT (datetime('now'))` for auto-populated, or `TEXT NOT NULL` for manual
- **Rust**: `chrono::DateTime<chrono::Utc>`
- **Format**: ISO 8601 (e.g., `2024-01-15T10:30:00Z`)

### Boolean Fields
- **JSON Schema**: `{ "type": "boolean" }`
- **SQLite**: `INTEGER NOT NULL DEFAULT 0` (0 = false, 1 = true)
- **Rust**: `bool`

### String Fields
- **JSON Schema**: `{ "type": "string" }`
- **SQLite**: `TEXT NOT NULL` or `TEXT` (if nullable)
- **Rust**: `String` or `Option<String>`

### Integer Fields
- **JSON Schema**: `{ "type": "integer" }`
- **SQLite**: `INTEGER NOT NULL` or `INTEGER` (if nullable)
- **Rust**: `i32`, `i64`, or `Option<i32>`

## Migration Templates

### up.sql Template
```sql
CREATE TABLE <table_name> (
    id TEXT PRIMARY KEY NOT NULL,
    -- other fields here
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Add indexes for frequently queried fields
CREATE INDEX idx_<table_name>_<field> ON <table_name>(<field>);
```

### down.sql Template
```sql
DROP INDEX IF EXISTS idx_<table_name>_<field>;
DROP TABLE IF EXISTS <table_name>;
```

## Example

Creating a "character" schema:

**shared/schemas/character.json**
```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "character.json",
  "title": "Character",
  "description": "A player character in the role-playing guild",
  "type": "object",
  "properties": {
    "id": {
      "type": "string",
      "format": "uuid",
      "description": "UUIDv4 identifier for the character"
    },
    "user_id": {
      "type": "string",
      "format": "uuid",
      "description": "UUIDv4 of the owning user"
    },
    "name": {
      "type": "string",
      "description": "Character name"
    },
    "created_at": {
      "type": "string",
      "format": "date-time",
      "description": "Timestamp when the character was created"
    },
    "updated_at": {
      "type": "string",
      "format": "date-time",
      "description": "Timestamp when the character was last updated"
    }
  },
  "required": ["id", "user_id", "name", "created_at", "updated_at"]
}
```

**backend/migrations/20260111120000_create_characters.up.sql**
```sql
CREATE TABLE characters (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL REFERENCES users(id),
    name TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_characters_user_id ON characters(user_id);
```

**backend/migrations/20260111120000_create_characters.down.sql**
```sql
DROP INDEX IF EXISTS idx_characters_user_id;
DROP TABLE IF EXISTS characters;
```
