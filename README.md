# Loafy Club - Pickleball Booking Platform

A comprehensive booking platform for Loafy Club, a pickleball club in Hanoi. Supports both local players (QR payments) and international players (Stripe), with subscriptions, waitlists, and advanced features.

## 📊 Project Status

**Current Phase**: Phase 1 - MVP Core Booking Flow
**Implementation**: 87% Complete (13/15 tasks)
**Status**: ✅ Ready for Testing

### ✅ Phase 1 - Completed Features
- ✅ User authentication via Supabase (Google, Facebook, Apple OAuth)
- ✅ Session management (create, view, update, delete) with role-based access
- ✅ Booking system with race condition protection (SELECT FOR UPDATE)
- ✅ Stripe payment integration with 30-minute payment deadline
- ✅ Background job to release unpaid bookings
- ✅ Full responsive frontend (auth, sessions, bookings, payments)
- ✅ Admin and organizer dashboards

### 🚧 Phase 1 - In Progress
- ⏳ Integration tests
- ⏳ End-to-end testing

### 📅 Future Phases
- **Phase 2**: Subscriptions + Waitlist (10 tickets for 800k VND, priority queue)
- **Phase 3**: QR payments with OCR, referrals, birthday bonuses, email notifications
- **Phase 4**: Production deployment, monitoring, security audit

See [PROGRESS.md](PROGRESS.md) for detailed progress tracking.

## Features

### Phase 1 (Current)
- ✅ **User Authentication**: OAuth via Google, Facebook, Apple (Supabase)
- ✅ **Session Management**: Create, view, update, delete sessions (organizer/admin)
- ✅ **Booking System**: Book for yourself + up to 3 guests
- ✅ **Stripe Payments**: Secure card payments with 30-minute deadline
- ✅ **Race Protection**: SELECT FOR UPDATE to prevent overbooking
- ✅ **Role-Based Access**: User, Organizer, Admin roles
- ✅ **Background Jobs**: Auto-release unpaid bookings after deadline

### Phase 2 (Planned)
- 🔜 **Subscription System**: 10 tickets for 800k VND (3 months, auto-renew)
- 🔜 **Early Access**: Subscribers get 7-day early booking window
- 🔜 **Waitlist**: Priority queue with staggered notifications
- 🔜 **Ticket Management**: Use tickets or pay per session

### Phase 3 (Planned)
- 🔜 **QR Payments**: Local bank transfer with OCR verification
- 🔜 **Referral System**: Bonus tickets for referrer and referee
- 🔜 **Birthday Bonuses**: Free ticket for active subscribers
- 🔜 **Email Notifications**: Daily recaps and booking alerts
- 🔜 **Guest Bookings**: Non-members can book at full price

### Phase 4 (Planned)
- 🔜 **Multi-language**: Full English and Vietnamese support
- 🔜 **Production Deployment**: Vultr VPS with Caddy HTTPS
- 🔜 **Monitoring**: Structured logging and error tracking
- 🔜 **Documentation**: OpenAPI specs and user guides

## Tech Stack

### Backend
- **Rust** with Axum web framework
- **SQLx** for type-safe database queries
- **PostgreSQL 16** database
- **tokio-cron-scheduler** for background jobs

### Frontend
- **SvelteKit** with TypeScript
- **Tailwind CSS** for styling
- **shadcn-svelte** for UI components
- **svelte-i18n** for internationalization

### Infrastructure
- **Docker Compose** for local development and production
- **Caddy** for reverse proxy with automatic HTTPS
- **Terraform** for Vultr VPS provisioning

### Third-party Services
- **Supabase** for authentication and file storage
- **Stripe** for payments and subscriptions
- **Google Cloud Vision** for OCR verification
- **Resend** for email notifications

## Local Development Setup

### Prerequisites

- **Rust** (latest stable): [Install Rust](https://rustup.rs/)
- **Node.js 18+**: [Install Node.js](https://nodejs.org/)
- **Docker & Docker Compose**: [Install Docker](https://docs.docker.com/get-docker/)
- **SQLx CLI**: `cargo install sqlx-cli --no-default-features --features postgres`

### Third-party Service Accounts

You'll need accounts for:
- [Supabase](https://supabase.com) (free tier)
- [Stripe](https://stripe.com) (test mode)
- [Google Cloud](https://console.cloud.google.com) (Cloud Vision API)
- [Resend](https://resend.com) (free tier)

See [docs/setup/](docs/setup/) for detailed setup instructions for each service.

### Initial Setup

```bash
# 1. Clone the repository
git clone https://github.com/Loafy-club/Booking.git
cd Booking

# 2. Set up environment variables
cp .env.example .env
# Edit .env with your service credentials

# 3. Start local PostgreSQL database
docker-compose -f infra/docker/docker-compose.dev.yml up -d

# 4. Run database migrations
cd backend
sqlx database create
sqlx migrate run

# 5. Install frontend dependencies
cd ../frontend
npm install

# 6. Generate TypeScript types from Rust
cd ..
./scripts/generate-types.sh
```

### Running the Application

You'll need three terminal windows:

**Terminal 1 - Backend API:**
```bash
cd backend
cargo run --bin loafy-api
# Server runs on http://localhost:3000
```

**Terminal 2 - Background Jobs:**
```bash
cd backend
cargo run --bin loafy-jobs
```

**Terminal 3 - Frontend:**
```bash
cd frontend
npm run dev
# Frontend runs on http://localhost:5173
```

### Testing

```bash
# Backend tests
cd backend
cargo test

# Frontend tests
cd frontend
npm test

# E2E tests
npm run test:e2e
```

## Project Structure

```
Booking/
├── backend/                    # Rust backend
│   ├── crates/
│   │   ├── api/               # Axum HTTP server
│   │   ├── core/              # Business logic
│   │   ├── db/                # Database models & queries
│   │   ├── jobs/              # Background jobs
│   │   ├── types/             # Shared types (with TS export)
│   │   └── integrations/      # Third-party integrations
│   └── migrations/            # SQLx database migrations
│
├── frontend/                   # SvelteKit frontend
│   └── src/
│       ├── routes/            # SvelteKit pages
│       └── lib/
│           ├── components/    # Svelte components
│           ├── api/           # API client
│           ├── stores/        # Svelte stores
│           ├── i18n/          # Translations (en, vi)
│           └── types/         # Generated TypeScript types
│
├── infra/                      # Infrastructure
│   ├── docker/                # Docker Compose files
│   └── terraform/             # Terraform for Vultr
│
├── docs/                       # Documentation
│   ├── setup/                 # Service setup guides
│   └── api/                   # API documentation
│
└── scripts/                    # Utility scripts
```

## Commands

### Development

```bash
# Backend
cargo run --bin loafy-api       # Start API server
cargo run --bin loafy-jobs      # Start background jobs
cargo test                      # Run tests
cargo fmt                       # Format code
cargo clippy                    # Lint code

# Frontend
npm run dev                     # Start dev server
npm run build                   # Build for production
npm test                        # Run tests
npm run lint                    # Lint code

# Database
sqlx migrate run                # Run migrations
sqlx migrate revert             # Revert last migration
sqlx migrate add <name>         # Create new migration

# Docker
docker-compose -f infra/docker/docker-compose.dev.yml up    # Start local services
docker-compose -f infra/docker/docker-compose.dev.yml down  # Stop local services
```

### Production

```bash
# Build and deploy with Docker Compose
cd infra/docker
docker-compose up -d --build

# Provision infrastructure with Terraform
cd infra/terraform
terraform init
terraform plan
terraform apply
```

## Deployment

See [docs/deployment.md](docs/deployment.md) for detailed deployment instructions.

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

MIT License - see [LICENSE](LICENSE) for details

## Support

For issues and questions:
- GitHub Issues: [github.com/Loafy-club/Booking/issues](https://github.com/Loafy-club/Booking/issues)
- Email: support@loafy.club

## Mascot

🐕 Corgi with white headband, holding pickleball paddle

---

**Loafy Club** - Pickleball in Hanoi 🏓
