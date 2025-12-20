# Loafy Club - Complete Project Context & Specifications

**DO NOT DELETE THIS FILE** - Contains all critical information needed to continue development from any point.

---

## Project Mission

Build a comprehensive pickleball booking platform for Loafy Club in Hanoi with dual payment systems (Stripe for international, QR with OCR for locals), subscriptions, waitlists, referrals, and email notifications.

---

## Critical Business Rules

### Pricing Model

**Drop-in Players:**
- Price: 100,000 VND per session
- Guests (+1s): 100,000 VND each (no discount)
- Queue priority: Last
- Cancellation deadline: 48h before session (configurable)
- Early booking: Not allowed (subscribers get 7-day early access)

**Subscribers:**
- Pack: 10 tickets for 800,000 VND (80k per ticket = 20% off)
- Validity: 3 months
- Auto-renew: Yes (default, cancellable)
- Guests: 100,000 VND each (no discount)
- Queue priority: First
- Early access: 7 days before drop-ins (configurable per session)
- Cancellation deadline: 24h before (configurable)
- Out-of-ticket discount: 10% off (90k VND) when tickets exhausted
- Referral bonus: Both parties get 1 ticket on first purchase
- Birthday bonus: 1 ticket (requires active subscription, account 30+ days old)

### Payment Deadline

- Users have **30 minutes** to complete payment after booking
- If unpaid after deadline: booking cancelled, slot returned to session
- Background job checks every 1 minute

### Waitlist Logic

When session is full:
1. Users can join waitlist
2. Priority: Subscribers first, then FIFO by join time
3. When slot opens:
   - Notify #1 → 1 hour exclusive access
   - After 1h: Notify #2 → both #1 and #2 can compete
   - After another 1h: Notify #3 → all three can compete
   - Continue pattern until someone books
4. First to complete payment wins the slot
5. Background job processes every 15 minutes

### Session Rules

- Created by users with **organizer** or **admin** role
- Multiple courts per session
- Default: 6 players per court (configurable globally and per-session)
- Total slots = courts × max_players_per_court
- Variable pricing (can override global defaults per session)
- Configurable early access hours per session
- Organizer uploads QR code for local payments (per session)

### Guest Bookings

- User can book for self + any number of guests
- **Guests ALWAYS pay full price** (100k VND each)
- Only the primary user can use subscription tickets
- Guests-only bookings allowed (user doesn't attend, only guests)

### Referral System

- Each user gets unique referral code
- New user signs up with code
- On first subscription purchase: **both** get 1 bonus ticket
- Tracked to prevent duplicate bonuses

### Birthday Bonus

- User sets birthday once (cannot change)
- **Account must be 30+ days old**
- **Active subscription required** (status = 'active')
- 1 ticket added on birthday (daily cron at 00:01)
- Tracked per year to prevent duplicates
- No bonus if subscription expired/cancelled

### QR Payment Verification (Phase 3)

**OCR Rules:**
- Booking code: Required
- Amount: Required (exact match)
- Date: Optional (within 24h if checked)
- Rate limiting: 3 uploads/hour, 5 uploads/day per user
- Monthly budget: 1000 OCR requests
- Alerts: 400 (40%), 700 (70%), 1000 (100% - OCR disabled)

**Auto-confirmation:**
- All required checks pass → auto_confirmed
- Any required check fails → pending_review (manual by organizer/admin)

**Screenshot retention:**
- Confirmed: Delete after 1 month
- Rejected: Delete immediately
- Cancelled booking: Delete immediately
- Timeout: Delete immediately

---

## User Roles & Permissions

### user (default)
- Book sessions for self + guests
- View and manage own bookings
- Purchase and manage subscriptions
- Join waitlists
- View booking history
- Set birthday (once)
- Share referral code

### organizer
- All user permissions
- Create sessions
- Edit own sessions
- Cancel own sessions (cannot delete)
- View bookings for own sessions
- Confirm/reject payments for own sessions
- Upload QR code for own sessions
- **Cannot** see other organizers' sessions
- **Cannot** see revenue/OCR stats

### admin
- All organizer permissions
- Manage ALL sessions (any organizer)
- View ALL bookings
- Confirm/reject ANY payment
- Delete sessions
- Manage users (change roles)
- View global stats and OCR usage
- Edit global configuration

### moderator
- Reserved for future use (TBD)

---

## Database Schema (Complete)

### Phase 1 Tables

```sql
-- Roles
roles: id (UUID), name (VARCHAR), created_at (TIMESTAMPTZ)
Default roles: 'user', 'organizer', 'admin', 'moderator'

-- Users
users: id (UUID), email (VARCHAR UNIQUE), name (VARCHAR), avatar_url (TEXT),
       phone (VARCHAR), role_id (UUID FK), auth_provider (VARCHAR),
       auth_provider_id (VARCHAR), created_at, updated_at

-- Config (global defaults)
config: id (UUID), key (VARCHAR UNIQUE), value (TEXT), description (TEXT), updated_at

Default config values:
- subscriber_early_access_hours: 168 (7 days)
- drop_in_price_vnd: 100000
- drop_in_cancellation_hours: 48
- subscriber_cancellation_hours: 24
- subscriber_out_of_ticket_discount_percent: 10
- payment_deadline_minutes: 30
- waitlist_exclusive_hours: 1
- max_players_per_court: 6
- birthday_account_age_days: 30
- referral_bonus_tickets: 1
- birthday_bonus_tickets: 1

-- Sessions
sessions: id (UUID), organizer_id (UUID FK), title (VARCHAR), date (DATE), time (TIME),
          location (TEXT), courts (INT), max_players_per_court (INT nullable),
          total_slots (INT), available_slots (INT),
          price_vnd (INT nullable), price_usd (DECIMAL nullable),
          subscriber_early_access_hours (INT nullable),
          drop_in_cancellation_hours (INT nullable),
          subscriber_cancellation_hours (INT nullable),
          qr_code_url (TEXT), cancelled (BOOLEAN), cancelled_at (TIMESTAMPTZ),
          created_at, updated_at

-- Bookings
bookings: id (UUID), user_id (UUID FK), session_id (UUID FK),
          booking_code (VARCHAR UNIQUE), guest_count (INT),
          tickets_used (INT), discount_applied (VARCHAR),
          price_paid_vnd (INT), price_paid_usd (DECIMAL),
          guest_price_paid_vnd (INT), guest_price_paid_usd (DECIMAL),
          payment_method (VARCHAR), payment_status (VARCHAR),
          verification_status (VARCHAR nullable),
          payment_screenshot_url (TEXT nullable),
          stripe_payment_id (VARCHAR nullable),
          payment_deadline (TIMESTAMPTZ nullable),
          cancelled_at (TIMESTAMPTZ nullable),
          created_at, updated_at
```

### Phase 2 Tables (Future)

```sql
-- Subscriptions
subscriptions: id, user_id, tickets_remaining, status, stripe_subscription_id,
               current_period_start, current_period_end, auto_renew, created_at

-- Waitlist
waitlist: id, user_id, session_id, priority (1=subscriber, 0=drop-in),
          position, notified_at, can_book, created_at

-- Notification Preferences
notification_preferences: id, user_id, email_alerts, daily_recap,
                          daily_recap_hour, in_app_notifications, created_at, updated_at

-- Notifications (in-app)
notifications: id, user_id, type, title, message, link, read, created_at
```

### Phase 3 Tables (Future)

```sql
-- Payment Screenshots
payment_screenshots: id, booking_id, file_url, uploaded_at

-- OCR Verifications
ocr_verifications: id, booking_id, raw_text, booking_code_found,
                   amount_found, date_found, checks_passed (JSONB), created_at

-- Monthly OCR Usage
monthly_ocr_usage: id, month (DATE UNIQUE), ocr_requests (INT),
                   alert_400_sent, alert_700_sent, alert_1000_sent, created_at

-- Screenshot Upload Attempts (rate limiting)
screenshot_upload_attempts: id, user_id, created_at

-- Referrals
referrals: id, referrer_id, referred_id, referrer_bonus_applied,
           referred_bonus_applied, created_at

-- Bonus Tickets Log
bonus_tickets: id, user_id, type ('referral'|'birthday'|'manual'),
               tickets (INT), note (TEXT), created_at

-- Auth Tokens (one-click email links)
auth_tokens: id, user_id, token (VARCHAR UNIQUE), target_url (TEXT),
             used (BOOLEAN), expires_at (TIMESTAMPTZ), created_at
```

---

## API Endpoints (Complete List)

### Phase 1 - Authentication
```
POST   /api/auth/callback       # OAuth callback from Supabase
GET    /api/auth/me             # Get current user
POST   /api/auth/logout         # Logout
```

### Phase 1 - Sessions
```
GET    /api/sessions            # List upcoming sessions (with filters)
GET    /api/sessions/:id        # Get session details
POST   /api/sessions            # Create session (organizer+)
PUT    /api/sessions/:id        # Update session (admin only)
DELETE /api/sessions/:id        # Delete session (admin only)
```

### Phase 1 - Bookings
```
GET    /api/bookings            # List my bookings
GET    /api/bookings/:id        # Get booking details
POST   /api/bookings            # Create booking (self + guests)
DELETE /api/bookings/:id        # Cancel booking
```

### Phase 1 - Payments (Stripe)
```
POST   /api/payments/stripe/intent     # Create payment intent
POST   /api/payments/stripe/confirm    # Confirm payment
POST   /api/webhooks/stripe            # Stripe webhooks
```

### Phase 2 - Subscriptions
```
GET    /api/subscriptions/current      # My subscription
POST   /api/subscriptions/purchase     # Buy subscription
POST   /api/subscriptions/cancel       # Cancel auto-renewal
GET    /api/subscriptions/tickets      # My ticket history
```

### Phase 2 - Waitlist
```
POST   /api/waitlist/join/:session_id  # Join waitlist
DELETE /api/waitlist/leave/:session_id # Leave waitlist
GET    /api/waitlist/:session_id       # My waitlist status
```

### Phase 2 - Notifications
```
GET    /api/notifications              # My notifications
PUT    /api/notifications/:id/read     # Mark as read
```

### Phase 3 - QR Payments
```
POST   /api/payments/qr/upload         # Upload screenshot
GET    /api/payments/qr/:booking_id    # QR payment status
POST   /api/payments/qr/:id/verify     # Admin: manual verification
```

### Phase 3 - Referrals
```
GET    /api/referrals/my-code          # My referral code
POST   /api/referrals/apply            # Apply referral code
```

### Phase 3 - Admin
```
GET    /api/admin/ocr-usage            # OCR usage stats
GET    /api/admin/payment-screenshots  # Pending verifications
POST   /api/admin/screenshots/:id/approve
POST   /api/admin/screenshots/:id/reject
GET    /api/admin/stats                # Dashboard statistics
GET    /api/admin/config               # View global config
PUT    /api/admin/config               # Update global config
POST   /api/admin/qr-code              # Upload global QR code
GET    /api/admin/users                # List users
PUT    /api/admin/users/:id/role       # Change user role
```

### Phase 3 - User Profile
```
PUT    /api/users/profile              # Update profile (birthday)
GET    /api/users/tickets              # Ticket history
```

---

## Background Jobs Schedule

```
Every 1 minute    → Release unpaid bookings (payment deadline)
Every 15 minutes  → Process waitlist (staggered notifications)
Every hour        → Stripe subscription sync
Daily at 00:01    → Birthday ticket allocation
Daily at 03:00    → Screenshot cleanup (1 month+)
Daily at 04:00    → Rate limit cleanup (7 days+)
1st of month      → Monthly OCR counter reset
Hourly at :00     → Daily recap emails (user-configured time)
```

---

## Tech Stack Details

### Backend (Rust)
- **Framework**: Axum 0.7 (async web framework on tokio)
- **Database**: SQLx 0.8 (compile-time checked queries, no ORM)
- **Runtime**: Tokio 1.42 (async runtime)
- **Jobs**: tokio-cron-scheduler 0.13 (cron-based background jobs)
- **Serialization**: serde 1.0 + serde_json
- **Validation**: validator 0.18 (derive macros)
- **Logging**: tracing + tracing-subscriber (structured logging)
- **OpenAPI**: utoipa 5.3 (auto-generate from Axum routes)

**Workspace Structure:**
```
backend/crates/
├── api/           # Axum HTTP server (main.rs, routes, middleware)
├── core/          # Business logic (booking, subscription, waitlist)
├── db/            # SQLx models & queries, connection pool
├── jobs/          # Background jobs (main.rs, scheduler)
├── types/         # Shared types, errors, API DTOs (ts-rs export)
└── integrations/  # Third-party: Stripe, Supabase, Google Vision, Resend
```

### Frontend (SvelteKit)
- **Framework**: SvelteKit (SSR/SPA with file-based routing)
- **Language**: TypeScript
- **Styling**: Tailwind CSS 3.x
- **Components**: shadcn-svelte (accessible UI components)
- **i18n**: svelte-i18n (English & Vietnamese)
- **HTTP**: Axios (with auth interceptors)
- **State**: Svelte stores (reactive state management)
- **Type Safety**: ts-rs (auto-generated types from Rust)

### Database
- **PostgreSQL 16** (alpine image)
- **Migrations**: SQLx CLI (`sqlx migrate`)
- **Connection Pool**: Max 20 connections

### Infrastructure
- **Local Dev**: Docker Compose (`infra/docker/docker-compose.dev.yml`)
- **Production**: Docker Compose (`infra/docker/docker-compose.yml`)
- **Reverse Proxy**: Caddy 2 (automatic HTTPS with Let's Encrypt)
- **Cloud**: Vultr VPS in Singapore (2 vCPU, 4GB RAM, ~$18/month)
- **IaC**: Terraform for provisioning

### Third-Party Services
- **Supabase** (Auth + Storage)
  - OAuth: Google, Facebook, Apple
  - Storage bucket: `payment-screenshots`
  - Region: Singapore
- **Stripe** (Payments + Subscriptions)
  - Products: Drop-in (100k VND), Subscription (800k VND/3mo)
  - Webhooks: payment_intent.*, customer.subscription.*, invoice.*
- **Google Cloud Vision** (OCR for QR payment screenshots)
  - Budget: 1000 requests/month
  - Alerts at 400, 700, 1000
- **Resend** (Email notifications)
  - Domain: loafy.club
  - Templates: Daily recap, payment confirmation, waitlist alerts

---

## Critical Implementation Patterns

### Race Condition Protection (Bookings)

```rust
// MUST use SELECT FOR UPDATE when creating bookings
async fn create_booking_with_lock(pool: &PgPool, session_id: Uuid) -> Result<Booking> {
    let mut tx = pool.begin().await?;

    // Lock session row
    let session = sqlx::query_as::<_, Session>(
        "SELECT * FROM sessions WHERE id = $1 FOR UPDATE"
    )
    .bind(session_id)
    .fetch_one(&mut *tx)
    .await?;

    // Check availability
    if session.available_slots < 1 {
        tx.rollback().await?;
        return Err(AppError::Conflict("No slots available".into()));
    }

    // Create booking
    let booking = sqlx::query_as::<_, Booking>(
        "INSERT INTO bookings (...) VALUES (...) RETURNING *"
    )
    .bind(...)
    .fetch_one(&mut *tx)
    .await?;

    // Decrement slots atomically
    sqlx::query("UPDATE sessions SET available_slots = available_slots - 1 WHERE id = $1")
        .bind(session_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(booking)
}
```

### Payment Deadline Calculation

```rust
use chrono::{Utc, Duration};

let payment_deadline = Utc::now() + Duration::minutes(
    config.payment_deadline_minutes.unwrap_or(30)
);
```

### Booking Code Generation

```rust
use rand::{distributions::Alphanumeric, Rng};

fn generate_booking_code() -> String {
    let suffix: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect();

    format!("LB-{}", suffix.to_uppercase())
}
```

### Stripe Webhook Signature Verification

```rust
use stripe::{Webhook, EventObject};

pub async fn handle_webhook(
    payload: String,
    signature: &str,
    webhook_secret: &str,
) -> Result<()> {
    // Verify signature
    let event = Webhook::construct_event(
        &payload,
        signature,
        webhook_secret,
    )?;

    // Track event ID for idempotency
    if event_already_processed(&event.id).await? {
        return Ok(()); // Already handled
    }

    match event.type_ {
        EventType::InvoicePaid => {
            // Allocate subscription tickets
        }
        EventType::PaymentIntentSucceeded => {
            // Confirm booking payment
        }
        // ... handle other events
    }

    mark_event_processed(&event.id).await?;
    Ok(())
}
```

---

## Environment Variables (Complete)

```bash
# Database
DATABASE_URL=postgresql://loafy:password@localhost:5432/loafy_booking

# Server
RUST_LOG=info,loafy_api=debug,loafy_jobs=debug
API_PORT=3000
FRONTEND_URL=http://localhost:5173

# Supabase
SUPABASE_URL=https://xxx.supabase.co
SUPABASE_ANON_KEY=eyJhbGc...
SUPABASE_SERVICE_KEY=eyJhbGc...

# Stripe
STRIPE_PUBLISHABLE_KEY=pk_test_...
STRIPE_SECRET_KEY=sk_test_...
STRIPE_WEBHOOK_SECRET=whsec_...

# Google Cloud Vision
GOOGLE_APPLICATION_CREDENTIALS=/path/to/credentials.json
GOOGLE_CLOUD_PROJECT=loafy-booking

# Email (Resend)
RESEND_API_KEY=re_...
FROM_EMAIL=noreply@loafy.club

# OCR Configuration
OCR_MONTHLY_BUDGET=1000
OCR_ALERT_THRESHOLD_1=400
OCR_ALERT_THRESHOLD_2=700

# Rate Limiting
SCREENSHOT_RATE_LIMIT_HOURLY=3
SCREENSHOT_RATE_LIMIT_DAILY=5

# Payment
PAYMENT_DEADLINE_MINUTES=30
BOOKING_CODE_PREFIX=LB

# App
APP_ENV=development
SECRET_KEY=your-secret-key-change-in-production
POSTGRES_PASSWORD=change-in-production
```

---

## Development Workflow

### Local Setup
```bash
# 1. Start PostgreSQL
docker-compose -f infra/docker/docker-compose.dev.yml up -d

# 2. Run migrations
cd backend && sqlx migrate run

# 3. Start API (terminal 1)
cargo run --bin loafy-api

# 4. Start jobs (terminal 2)
cargo run --bin loafy-jobs

# 5. Start frontend (terminal 3)
cd frontend && npm run dev
```

### Create Database Migration
```bash
cd backend
sqlx migrate add description_of_change
# Edit backend/migrations/TIMESTAMP_description_of_change.sql
sqlx migrate run
cargo sqlx prepare  # For offline builds
```

### Generate TypeScript Types
```bash
./scripts/generate-types.sh
# Or manually:
cd backend && cargo test --features ts-rs
```

### Run Tests
```bash
# Backend
cd backend && cargo test

# Frontend
cd frontend && npm test
```

---

## Current Implementation Status

**Phase**: Phase 1 - MVP Core Booking Flow
**Started**: December 20, 2024
**Status**: In Progress

**Completed**:
- ✅ Project structure initialized
- ✅ Rust workspace with 6 crates
- ✅ SvelteKit frontend with TypeScript
- ✅ Docker Compose configurations
- ✅ Initial database migration (Phase 1 schema)
- ✅ Environment templates
- ✅ Documentation (README, CLAUDE.md, PROGRESS.md, CONTEXT.md)
- ✅ GitHub repository created: https://github.com/Loafy-club/Booking
- ✅ Supabase Auth integration
- ✅ User database models and queries

**In Progress**:
- ⏳ Auth middleware and routes
- ⏳ Session management
- ⏳ Booking system
- ⏳ Stripe integration
- ⏳ Frontend implementation

**Next Steps**:
- Complete Phase 1 (target: ~1 week)
- Deploy Phase 1 to Vultr
- Begin Phase 2 (Subscriptions + Waitlist)

---

## Repository Structure

```
Booking/
├── .github/workflows/          # CI/CD (optional)
├── backend/
│   ├── Cargo.toml             # Workspace manifest
│   ├── Dockerfile             # API production build
│   ├── Dockerfile.jobs        # Jobs production build
│   ├── migrations/            # SQLx migrations
│   │   └── 20240101000000_initial_schema.sql
│   └── crates/
│       ├── api/               # HTTP server (Axum routes)
│       ├── core/              # Business logic
│       ├── db/                # Database layer (SQLx)
│       ├── jobs/              # Background jobs
│       ├── types/             # Shared types (ts-rs)
│       └── integrations/      # Third-party clients
├── frontend/
│   ├── Dockerfile             # Production build
│   ├── src/
│   │   ├── routes/            # SvelteKit pages
│   │   └── lib/
│   │       ├── components/    # Svelte components
│   │       ├── api/           # API client
│   │       ├── stores/        # State management
│   │       ├── i18n/          # Translations (en, vi)
│   │       └── types/         # Generated from Rust
│   ├── package.json
│   └── tailwind.config.js
├── infra/
│   ├── docker/
│   │   ├── docker-compose.dev.yml   # Local dev
│   │   ├── docker-compose.yml       # Production
│   │   └── caddy/Caddyfile          # Reverse proxy
│   └── terraform/             # Vultr provisioning
├── docs/
│   └── setup/                 # Service setup guides
├── scripts/
│   ├── setup-local.sh         # Automated setup
│   └── generate-types.sh      # TS type generation
├── .env.example               # Environment template
├── .gitignore
├── README.md                  # Project overview
├── CLAUDE.md                  # Development guide
├── PROGRESS.md                # Detailed task tracker
├── CONTEXT.md                 # THIS FILE - Complete specs
└── LICENSE                    # MIT
```

---

## Testing Strategy

### Unit Tests
- Booking validation logic
- Price calculation
- Booking code generation
- Authorization checks
- Date/time utilities

### Integration Tests
- Full booking flow with database
- Concurrent booking attempts (race conditions)
- Payment webhook handling
- Session CRUD with permissions

### E2E Tests (Playwright)
- User signup → book → pay → confirmed
- Organizer creates session
- Subscription purchase → book with ticket
- Waitlist join → notification → claim

---

## Deployment Strategy

### Development
- Docker Compose with PostgreSQL only
- Hot reload: cargo watch, Vite dev server

### Production
- Vultr VPS (Singapore, $18/month)
- Docker Compose (all services)
- Caddy (auto HTTPS)
- PostgreSQL in container with volumes
- Automated backups to Vultr object storage

**Deployment Process:**
1. Build Docker images
2. Push to GitHub Container Registry
3. SSH to server
4. Pull images
5. Run migrations: `sqlx migrate run`
6. Restart: `docker-compose up -d`
7. Smoke test

---

## Critical Reminders

1. **ALWAYS use SELECT FOR UPDATE** for booking creation
2. **Verify Stripe webhook signatures** (security)
3. **Track event IDs for idempotency** (Stripe webhooks)
4. **Guests ALWAYS pay full price** (no subscription discount)
5. **Birthday bonus requires active subscription**
6. **OCR budget enforcement** (stop at 1000/month)
7. **Screenshot rate limiting** (3/hour, 5/day)
8. **Payment deadline is 30 minutes** (configurable)
9. **Waitlist has staggered notifications** (1h exclusive pattern)
10. **Organizers can only see their own sessions** (not others')

---

**Last Updated**: December 20, 2024
**Version**: 1.0.0
**Status**: Phase 1 In Progress
