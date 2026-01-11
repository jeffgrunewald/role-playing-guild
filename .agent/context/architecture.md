| name | purpose |
|------|---------|
| architecture | document current architecture for agents |

# Architecture Context

@docs/ARCHITECTURE.md

## Quick Reference

- **Backend**: Rust + Tokio + Axum + SQLite (via SQLx)
- **Mobile**: Flutter (Guild Pass)
- **API**: RESTful JSON
- **Database**: SQLite single-file for portability

## Key Directories

| Path | Purpose |
|------|---------|
| `backend/` | Rust web server |
| `backend/migrations/` | SQLx database migrations |
| `mobile/` | Guild Pass Flutter app |
| `shared/schemas/` | JSON Schema definitions (shared across apps) |
| `shared/api-spec/` | API specifications |

## Schema Conventions

All entities must include:
- `id`: UUIDv4 (TEXT in SQLite)
- `created_at`: ISO 8601 timestamp (TEXT)
- `updated_at`: ISO 8601 timestamp (TEXT)

See @.agent/skills/create-schema.md for schema creation process.
