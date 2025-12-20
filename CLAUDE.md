# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Loafy Club booking platform - a full-stack pickleball session booking system for a club in Hanoi. Supports local payments (QR with OCR verification) and international payments (Stripe), with subscriptions, waitlists, referrals, and email notifications.

## Tech Stack

- **Backend**: Rust workspace with Axum, SQLx, tokio-cron-scheduler
- **Frontend**: SvelteKit with TypeScript, Tailwind CSS, shadcn-svelte
- **Database**: PostgreSQL 16
- **Type sharing**: ts-rs (Rust → TypeScript generation)
- **Third-party**: Supabase Auth/Storage, Stripe, Google Cloud Vision, Resend

## Development Commands

### Backend (from `/backend`)

```bash
# Run API server (port 3000)
cargo run --bin loafy-api

# Run background jobs
cargo run --bin loafy-jobs

# Run tests
cargo test

# Run specific crate tests
cargo test -p loafy-db

# Run with logs
RUST_LOG=debug cargo run --bin loafy-api

# Database migrations
sqlx migrate run                  # Apply migrations
sqlx migrate revert               # Revert last migration
sqlx migrate add <description>    # Create new migration
cargo sqlx prepare                # Generate sqlx-data.json for offline builds
```

### Frontend (from `/frontend`)

```bash
# Development server (port 5173)
npm run dev

# Build for production
npm run build

# Type check
npm run check

# Generate TS types from Rust (from project root)
cd ../scripts && ./generate-types.sh
```

### Docker

```bash
# Start local PostgreSQL
docker-compose -f infra/docker/docker-compose.dev.yml up -d

# Stop local services
docker-compose -f infra/docker/docker-compose.dev.yml down

# Production build
docker-compose -f infra/docker/docker-compose.yml up -d --build
```

## Architecture

### Workspace Structure

**Backend crates** (in `backend/crates/`):
- `api`: Axum HTTP server, routes, middleware, main.rs
- `core`: Business logic (booking, subscription, waitlist, payment logic)
- `db`: SQLx models and queries, connection pool
- `jobs`: Background jobs (tokio-cron-scheduler), separate main.rs
- `types`: Shared types with ts-rs export for frontend, error types
- `integrations`: Third-party services (Supabase, Stripe, Google Vision, Resend)

**Key dependency flow**: api → core → db ← types; jobs → core → db

### Database

**Connection**: SQLx with compile-time checked queries. Connection pool created in `db/src/pool.rs`.

**Migrations**: Located in `backend/migrations/`. Use SQLx CLI to manage. Naming: `YYYYMMDDHHMMSS_description.sql`

**Critical patterns**:
- Use `SELECT ... FOR UPDATE` for booking race condition protection (see `db/src/queries/bookings.rs`)
- Transactions for multi-table operations
- Indexes on frequently queried columns (user_id, session_id, status, etc.)

### Type Sharing

Rust structs in `types/src/api/` are exported to TypeScript using ts-rs:

```rust
#[derive(Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct SessionResponse { ... }
```

Run `./scripts/generate-types.sh` after modifying Rust types to update frontend types.

### Critical Business Logic

**Booking Race Conditions** (`backend/crates/core/src/booking/`):
- MUST use database-level locking (`SELECT FOR UPDATE`) when checking availability
- Decrement `available_slots` atomically in transaction
- Handle concurrent bookings gracefully

**Payment Deadline Enforcement** (`backend/crates/jobs/src/jobs/release_unpaid.rs`):
- Job runs every 1 minute
- Cancels bookings with `payment_status = 'pending'` older than deadline
- Returns slots to session atomically

**Waitlist Processing** (`backend/crates/jobs/src/jobs/process_waitlist.rs`):
- Job runs every 15 minutes
- Implements staggered notifications (1h exclusive, then progressive unlock)
- Priority: subscribers first, then FIFO
- Must track notification state in database

**OCR Verification** (`backend/crates/integrations/src/google_vision/`):
- Verify booking code, amount, and date from screenshot
- Track monthly usage (1000/month budget)
- Rate limiting: 3/hour, 5/day per user
- Auto-confirm or flag for manual review

**Stripe Webhooks** (`backend/crates/integrations/src/stripe/webhooks.rs`):
- Verify webhook signatures
- Handle: `invoice.paid`, `invoice.payment_failed`, `customer.subscription.*`
- Use idempotency (track processed event IDs)
- Return 200 immediately, process async

### API Structure

Routes in `backend/crates/api/src/routes/`:
- `auth.rs`: OAuth callback, user session
- `sessions.rs`: CRUD for sessions
- `bookings.rs`: Create, cancel, list bookings
- `subscriptions.rs`: Purchase, cancel subscriptions
- `payments.rs`: Stripe intents, QR upload
- `admin.rs`: Admin-only endpoints
- `notifications.rs`: In-app notifications

**Middleware**:
- Auth: Extract user from Supabase JWT
- CORS: Allow frontend origin
- Logging: tracing with request IDs

### Frontend Structure

**Routes** (`frontend/src/routes/`):
- SvelteKit file-based routing
- `/sessions` - Browse sessions
- `/sessions/[id]` - Session detail + booking
- `/bookings` - My bookings
- `/subscriptions` - Subscription management
- `/admin/*` - Admin pages
- `/organizer/*` - Organizer pages

**Components** (`frontend/src/lib/components/`):
- Reusable Svelte components
- shadcn-svelte UI primitives
- Custom: SessionCard, BookingList, PaymentForm, QRPaymentUpload

**API Client** (`frontend/src/lib/api/`):
- Axios-based client with auth interceptors
- Auto-includes Supabase JWT in headers
- Organized by domain: `sessions.ts`, `bookings.ts`, etc.

**Stores** (`frontend/src/lib/stores/`):
- Svelte stores for global state
- `auth.ts`: Current user, session
- `notifications.ts`: In-app notifications

**i18n** (`frontend/src/lib/i18n/`):
- `en.json` and `vi.json` for translations
- Use `$t('key')` in components

## Testing

**Backend**:
- Unit tests: In module with `#[cfg(test)]`
- Integration tests: `backend/tests/` with `#[sqlx::test]` macro
- Test with real PostgreSQL (uses test DB)

**Frontend**:
- Unit: Vitest
- E2E: Playwright (critical user flows)

**Key test scenarios**:
- Concurrent booking attempts (race conditions)
- Payment deadline enforcement
- Waitlist notification timing
- OCR verification with various inputs
- Subscription renewal flows

## Common Tasks

### Add a new API endpoint

1. Define request/response types in `backend/crates/types/src/api/`
2. Add database queries in `backend/crates/db/src/queries/`
3. Implement business logic in `backend/crates/core/src/`
4. Add route handler in `backend/crates/api/src/routes/`
5. Register route in `main.rs`
6. Generate TypeScript types: `./scripts/generate-types.sh`
7. Call from frontend: `frontend/src/lib/api/`

### Add a background job

1. Create job file in `backend/crates/jobs/src/jobs/`
2. Implement job logic with database operations
3. Register with cron scheduler in `main.rs`
4. Add configuration to `.env` if needed
5. Test job execution and error handling

### Add a database migration

1. Create migration: `sqlx migrate add <description>`
2. Write SQL in `backend/migrations/TIMESTAMP_description.sql`
3. Test migration: `sqlx migrate run`
4. Test rollback if reversible
5. Update models in `backend/crates/db/src/models/`
6. Update queries as needed
7. Run `cargo sqlx prepare` for offline builds

### Add a third-party integration

1. Add integration module in `backend/crates/integrations/src/`
2. Add dependencies to `integrations/Cargo.toml`
3. Add configuration to `.env.example`
4. Add setup docs to `docs/setup/`
5. Implement client with error handling
6. Add integration tests

## Important Notes

### Environment Variables

Required in `.env` (copy from `.env.example`):
- `DATABASE_URL`: PostgreSQL connection string
- `SUPABASE_URL`, `SUPABASE_ANON_KEY`, `SUPABASE_SERVICE_KEY`
- `STRIPE_SECRET_KEY`, `STRIPE_WEBHOOK_SECRET`
- `GOOGLE_APPLICATION_CREDENTIALS` (path to JSON key)
- `RESEND_API_KEY`

### Database Constraints

- `available_slots` must be >= 0 (enforced in application)
- Payment deadline stored as `TIMESTAMPTZ` (use database `NOW()`)
- Booking codes must be unique (database constraint)
- Email must be unique per user

### API Rate Limits

- Screenshot uploads: 3/hour, 5/day per user (tracked in `screenshot_upload_attempts`)
- OCR budget: 1000/month (tracked in `monthly_ocr_usage`)
- Alerts at 400, 700 calls (sent to admin)

### Security

- NEVER store credit cards (Stripe handles)
- Webhook signature verification required
- SQL injection prevented by SQLx parameterized queries
- XSS prevented by Svelte auto-escaping
- Auth: Supabase JWT validated on every request
- HTTPS enforced in production (Caddy)

### Performance

- Database: Connection pool max 20
- Pagination: Default 20 items per page
- Background jobs: Run at staggered intervals to avoid load spikes
- Consider caching for session lists (future)

## Debugging

**Backend logs**:
```bash
RUST_LOG=debug,sqlx=info cargo run --bin loafy-api
```

**Database queries**:
- SQLx logs all queries with `RUST_LOG=sqlx=debug`
- Use `EXPLAIN ANALYZE` for slow queries

**Background jobs**:
- Check job scheduler logs: `RUST_LOG=loafy_jobs=debug cargo run --bin loafy-jobs`
- Jobs log start/end times and errors

**Frontend**:
- Browser dev tools for API calls
- SvelteKit dev server shows route load errors

## Deployment

**Development**: Docker Compose (`infra/docker/docker-compose.dev.yml`) - PostgreSQL only

**Production**: Docker Compose (`infra/docker/docker-compose.yml`) - All services + Caddy

**Infrastructure**: Terraform for Vultr VPS (`infra/terraform/`)

**Deployment steps**:
1. Build Docker images
2. Push to registry
3. SSH to server, pull images
4. Run migrations: `sqlx migrate run`
5. Restart services: `docker-compose up -d`

## Troubleshooting

**SQLx compile errors**: Run `cargo sqlx prepare` to generate offline query data

**Type mismatch between Rust/TS**: Regenerate types with `./scripts/generate-types.sh`

**Migration conflicts**: Ensure migrations are sequential, revert and recreate if needed

**Background job not running**: Check cron expression, ensure job is registered in scheduler

**Stripe webhook failures**: Verify webhook secret, check signature validation

**OCR quota exceeded**: Check `monthly_ocr_usage` table, alerts sent at thresholds

## Phase 1 Status

Currently in **Phase 1 MVP** implementation:
- ✅ Project structure initialized
- ✅ Database schema (roles, users, sessions, bookings, config)
- ⏳ Auth integration (Supabase OAuth)
- ⏳ Session CRUD endpoints
- ⏳ Booking creation with race protection
- ⏳ Stripe payment flow
- ⏳ Release unpaid bookings job
- ⏳ Frontend pages (sessions, bookings)

**Next steps**: Complete Phase 1, then Phase 2 (subscriptions + waitlist), then Phase 3 (OCR, referrals, emails).
