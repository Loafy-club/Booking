# Loafy Club - Development Progress Tracker

**Current Phase**: Phase 1 - MVP Core Booking Flow
**Started**: December 20, 2024
**Status**: 🚧 In Progress

---

## Phase 1: MVP - Core Booking Flow

**Goal**: Users can view sessions and make bookings with Stripe payments

### 🎯 Objectives
- ✅ Database schema (roles, users, sessions, bookings, config)
- ⏳ Supabase OAuth authentication
- ⏳ Session management (CRUD)
- ⏳ Booking creation with race condition protection
- ⏳ Stripe payment integration
- ⏳ Payment deadline enforcement
- ⏳ Frontend pages and flows

---

## 📋 Detailed Task List

### 1. Backend - Authentication & Authorization (0/4 completed)

#### 1.1 Supabase Integration ⏳
- [ ] `backend/crates/integrations/src/supabase/auth.rs`
  - [ ] Supabase client initialization
  - [ ] Verify JWT token function
  - [ ] Get user from token function
  - [ ] OAuth callback handler

- [ ] `backend/crates/integrations/src/supabase/storage.rs`
  - [ ] Storage client initialization (for future QR uploads)
  - [ ] Upload file function
  - [ ] Delete file function

#### 1.2 Auth Middleware ⏳
- [ ] `backend/crates/api/src/middleware/auth.rs`
  - [ ] JWT extractor from Authorization header
  - [ ] User extractor (validates + fetches user from DB)
  - [ ] Optional user extractor (for public endpoints)

- [ ] `backend/crates/api/src/middleware/mod.rs`
  - [ ] Export auth middleware
  - [ ] CORS middleware
  - [ ] Logging middleware

#### 1.3 Auth Routes ⏳
- [ ] `backend/crates/api/src/routes/auth.rs`
  - [ ] POST `/api/auth/callback` - Handle OAuth callback
  - [ ] GET `/api/auth/me` - Get current user
  - [ ] POST `/api/auth/logout` - Logout user

#### 1.4 User Database Queries ⏳
- [ ] `backend/crates/db/src/queries/users.rs`
  - [ ] `find_by_email()`
  - [ ] `find_by_id()`
  - [ ] `create_user()`
  - [ ] `update_user()`
  - [ ] `get_user_role()`

---

### 2. Backend - Session Management (0/5 completed)

#### 2.1 Session Database Models ⏳
- [ ] `backend/crates/db/src/models/session.rs`
  - [ ] Session struct with all fields
  - [ ] From database row implementation

#### 2.2 Session Database Queries ⏳
- [ ] `backend/crates/db/src/queries/sessions.rs`
  - [ ] `list_sessions()` - with filters (date, organizer, availability)
  - [ ] `get_session_by_id()`
  - [ ] `create_session()`
  - [ ] `update_session()`
  - [ ] `delete_session()`
  - [ ] `get_session_with_organizer()` - joined query
  - [ ] `decrement_available_slots()` - atomic update
  - [ ] `increment_available_slots()` - atomic update

#### 2.3 Session Business Logic ⏳
- [ ] `backend/crates/core/src/session/mod.rs`
  - [ ] Calculate total slots from courts × max_players
  - [ ] Validate session dates (must be in future)
  - [ ] Check user permissions (organizer/admin)

#### 2.4 Session API Routes ⏳
- [ ] `backend/crates/api/src/routes/sessions.rs`
  - [ ] GET `/api/sessions` - List all upcoming sessions
  - [ ] GET `/api/sessions/:id` - Get session details
  - [ ] POST `/api/sessions` - Create session (organizer+)
  - [ ] PUT `/api/sessions/:id` - Update session (admin only)
  - [ ] DELETE `/api/sessions/:id` - Delete session (admin only)

#### 2.5 Session Integration ⏳
- [ ] Register routes in `main.rs`
- [ ] Add authorization checks
- [ ] Test with curl/Postman

---

### 3. Backend - Booking System (0/6 completed)

#### 3.1 Booking Database Models ⏳
- [ ] `backend/crates/db/src/models/booking.rs`
  - [ ] Booking struct with all fields
  - [ ] From database row implementation

#### 3.2 Booking Database Queries ⏳
- [ ] `backend/crates/db/src/queries/bookings.rs`
  - [ ] `create_booking_with_lock()` - SELECT FOR UPDATE pattern
  - [ ] `get_booking_by_id()`
  - [ ] `get_booking_by_code()`
  - [ ] `list_user_bookings()`
  - [ ] `list_session_bookings()`
  - [ ] `cancel_booking()`
  - [ ] `update_payment_status()`
  - [ ] `find_unpaid_expired_bookings()` - for background job

#### 3.3 Booking Business Logic ⏳
- [ ] `backend/crates/core/src/booking/create.rs`
  - [ ] Generate unique booking code (LB-XXXXX)
  - [ ] Calculate price (drop-in base price + guests)
  - [ ] Check session availability (atomic check)
  - [ ] Validate guest count
  - [ ] Calculate payment deadline (30 minutes from now)

- [ ] `backend/crates/core/src/booking/cancel.rs`
  - [ ] Check cancellation deadline
  - [ ] Return slots to session
  - [ ] Handle refunds (if paid)

- [ ] `backend/crates/core/src/booking/validation.rs`
  - [ ] Validate booking request
  - [ ] Check session not cancelled
  - [ ] Check session not in past

#### 3.4 Booking API Routes ⏳
- [ ] `backend/crates/api/src/routes/bookings.rs`
  - [ ] GET `/api/bookings` - List my bookings
  - [ ] GET `/api/bookings/:id` - Get booking details
  - [ ] POST `/api/bookings` - Create booking
  - [ ] DELETE `/api/bookings/:id` - Cancel booking

#### 3.5 Race Condition Protection ⏳
- [ ] Implement transaction wrapper
- [ ] Test concurrent booking attempts
- [ ] Verify slot count integrity

#### 3.6 Booking Integration ⏳
- [ ] Register routes in `main.rs`
- [ ] Add authorization checks
- [ ] Test booking flow end-to-end

---

### 4. Backend - Stripe Payment Integration (0/5 completed)

#### 4.1 Stripe Client Setup ⏳
- [ ] `backend/crates/integrations/src/stripe/mod.rs`
  - [ ] Initialize Stripe client with secret key
  - [ ] Export payment and webhook modules

#### 4.2 Stripe Payment Logic ⏳
- [ ] `backend/crates/integrations/src/stripe/payments.rs`
  - [ ] `create_payment_intent()` - for booking
  - [ ] `confirm_payment()` - after user completes
  - [ ] `refund_payment()` - for cancellations
  - [ ] `get_payment_status()`

#### 4.3 Stripe Webhook Handling ⏳
- [ ] `backend/crates/integrations/src/stripe/webhooks.rs`
  - [ ] Verify webhook signature
  - [ ] Handle `payment_intent.succeeded`
  - [ ] Handle `payment_intent.failed`
  - [ ] Update booking payment status
  - [ ] Track processed event IDs (idempotency)

#### 4.4 Payment API Routes ⏳
- [ ] `backend/crates/api/src/routes/payments.rs`
  - [ ] POST `/api/payments/stripe/intent` - Create payment intent
  - [ ] POST `/api/payments/stripe/confirm` - Confirm payment
  - [ ] POST `/api/webhooks/stripe` - Stripe webhook endpoint

#### 4.5 Payment Integration ⏳
- [ ] Register routes in `main.rs`
- [ ] Test payment flow with Stripe test cards
- [ ] Test webhook handling with Stripe CLI

---

### 5. Backend - Background Jobs (0/3 completed)

#### 5.1 Job Scheduler Setup ⏳
- [ ] `backend/crates/jobs/src/scheduler.rs`
  - [ ] Initialize tokio-cron-scheduler
  - [ ] Register all jobs
  - [ ] Error handling and logging

#### 5.2 Release Unpaid Bookings Job ⏳
- [ ] `backend/crates/jobs/src/jobs/release_unpaid.rs`
  - [ ] Find bookings past payment deadline
  - [ ] Set status to 'cancelled'
  - [ ] Return slots to sessions atomically
  - [ ] Log cancelled bookings
  - [ ] Cron: every 1 minute

#### 5.3 Jobs Integration ⏳
- [ ] Update `main.rs` to start scheduler
- [ ] Test job execution
- [ ] Verify slot return works correctly

---

### 6. Frontend - Setup & Configuration (0/4 completed)

#### 6.1 Tailwind CSS Configuration ⏳
- [ ] `frontend/tailwind.config.js`
  - [ ] Configure content paths
  - [ ] Add custom colors (Loafy brand)
  - [ ] Add custom fonts

- [ ] `frontend/postcss.config.js`
  - [ ] Add Tailwind plugins

- [ ] `frontend/src/app.css`
  - [ ] Import Tailwind directives
  - [ ] Add global styles

#### 6.2 shadcn-svelte Components ⏳
- [ ] Install shadcn-svelte CLI
- [ ] Add components: Button, Card, Input, Form, Table, Badge, Alert
- [ ] Configure components theme

#### 6.3 API Client Setup ⏳
- [ ] `frontend/src/lib/api/client.ts`
  - [ ] Axios instance with base URL
  - [ ] Auth interceptor (add JWT to headers)
  - [ ] Response interceptor (handle errors)
  - [ ] Refresh token logic

#### 6.4 i18n Setup ⏳
- [ ] `frontend/src/lib/i18n/index.ts`
  - [ ] Initialize svelte-i18n
  - [ ] Language switcher

- [ ] `frontend/src/lib/i18n/en.json`
  - [ ] English translations (Phase 1 keys)

- [ ] `frontend/src/lib/i18n/vi.json`
  - [ ] Vietnamese translations (Phase 1 keys)

---

### 7. Frontend - Authentication (0/5 completed)

#### 7.1 Auth Store ⏳
- [ ] `frontend/src/lib/stores/auth.ts`
  - [ ] Svelte store for current user
  - [ ] Supabase client initialization
  - [ ] Login function (OAuth redirect)
  - [ ] Logout function
  - [ ] Get current user function
  - [ ] Session persistence (localStorage)

#### 7.2 Auth API Client ⏳
- [ ] `frontend/src/lib/api/auth.ts`
  - [ ] `handleCallback()` - process OAuth callback
  - [ ] `getCurrentUser()`
  - [ ] `logout()`

#### 7.3 Auth Pages ⏳
- [ ] `frontend/src/routes/auth/login/+page.svelte`
  - [ ] OAuth buttons (Google, Facebook, Apple)
  - [ ] Redirect to Supabase OAuth

- [ ] `frontend/src/routes/auth/callback/+page.svelte`
  - [ ] Get token from URL
  - [ ] Call backend callback endpoint
  - [ ] Store user in store
  - [ ] Redirect to home

- [ ] `frontend/src/routes/auth/logout/+page.svelte`
  - [ ] Clear session
  - [ ] Redirect to login

#### 7.4 Protected Routes ⏳
- [ ] `frontend/src/hooks.server.ts`
  - [ ] Check authentication on protected routes
  - [ ] Redirect to login if not authenticated

#### 7.5 Auth Components ⏳
- [ ] `frontend/src/lib/components/AuthGuard.svelte`
  - [ ] Wrapper for protected pages

- [ ] `frontend/src/lib/components/Header.svelte`
  - [ ] Logo, navigation, user menu
  - [ ] Login/Logout button

---

### 8. Frontend - Sessions (0/5 completed)

#### 8.1 Sessions API Client ⏳
- [ ] `frontend/src/lib/api/sessions.ts`
  - [ ] `listSessions(filters)`
  - [ ] `getSession(id)`
  - [ ] `createSession(data)` - organizer
  - [ ] `updateSession(id, data)` - admin
  - [ ] `deleteSession(id)` - admin

#### 8.2 Session Components ⏳
- [ ] `frontend/src/lib/components/SessionCard.svelte`
  - [ ] Display session info (title, date, time, location)
  - [ ] Show available slots
  - [ ] Book button
  - [ ] Props: session object

- [ ] `frontend/src/lib/components/SessionList.svelte`
  - [ ] List of SessionCard components
  - [ ] Empty state
  - [ ] Loading state

#### 8.3 Sessions Pages ⏳
- [ ] `frontend/src/routes/sessions/+page.svelte`
  - [ ] List upcoming sessions
  - [ ] Filter by date
  - [ ] Search by location/title

- [ ] `frontend/src/routes/sessions/[id]/+page.svelte`
  - [ ] Session details
  - [ ] Available slots indicator
  - [ ] Book button (redirects to booking flow)
  - [ ] Organizer info

#### 8.4 Organizer Session Creation ⏳
- [ ] `frontend/src/routes/organizer/sessions/create/+page.svelte`
  - [ ] Form: title, date, time, location, courts
  - [ ] Calculate total slots preview
  - [ ] Submit → create session
  - [ ] Redirect to session detail

#### 8.5 Admin Session Management ⏳
- [ ] `frontend/src/routes/admin/sessions/+page.svelte`
  - [ ] List all sessions (including past/cancelled)
  - [ ] Edit/Delete buttons
  - [ ] Quick stats

---

### 9. Frontend - Bookings (0/5 completed)

#### 9.1 Bookings API Client ⏳
- [ ] `frontend/src/lib/api/bookings.ts`
  - [ ] `createBooking(sessionId, guestCount, paymentMethod)`
  - [ ] `listMyBookings()`
  - [ ] `getBooking(id)`
  - [ ] `cancelBooking(id)`

#### 9.2 Booking Components ⏳
- [ ] `frontend/src/lib/components/BookingForm.svelte`
  - [ ] Guest count selector
  - [ ] Price calculation display
  - [ ] Payment method selector (Stripe only for Phase 1)
  - [ ] Submit button

- [ ] `frontend/src/lib/components/BookingList.svelte`
  - [ ] List user's bookings
  - [ ] Status badges (pending, confirmed, cancelled)
  - [ ] Booking code display

#### 9.3 Booking Pages ⏳
- [ ] `frontend/src/routes/bookings/+page.svelte`
  - [ ] My bookings list
  - [ ] Filter by status
  - [ ] Upcoming vs past bookings

- [ ] `frontend/src/routes/bookings/[id]/+page.svelte`
  - [ ] Booking details
  - [ ] Session info
  - [ ] Payment status
  - [ ] Cancel button (if allowed)
  - [ ] Booking code (for check-in)

#### 9.4 Booking Flow ⏳
- [ ] `frontend/src/routes/sessions/[id]/book/+page.svelte`
  - [ ] Booking form
  - [ ] Create booking → redirect to payment

#### 9.5 Payment Flow ⏳
- [ ] `frontend/src/routes/bookings/[id]/payment/+page.svelte`
  - [ ] Stripe Elements integration
  - [ ] Payment form
  - [ ] Confirm payment
  - [ ] Show payment deadline countdown
  - [ ] Redirect to confirmation on success

---

### 10. Frontend - Home & Layout (0/3 completed)

#### 10.1 Root Layout ⏳
- [ ] `frontend/src/routes/+layout.svelte`
  - [ ] Header with navigation
  - [ ] Footer
  - [ ] Toast notifications container
  - [ ] Language switcher

#### 10.2 Home Page ⏳
- [ ] `frontend/src/routes/+page.svelte`
  - [ ] Hero section with mascot (Corgi)
  - [ ] Upcoming sessions preview
  - [ ] CTA: Browse sessions / Login

#### 10.3 Error Pages ⏳
- [ ] `frontend/src/routes/+error.svelte`
  - [ ] 404 not found
  - [ ] 500 server error
  - [ ] User-friendly messages

---

### 11. Testing & Quality (0/4 completed)

#### 11.1 Backend Unit Tests ⏳
- [ ] Test booking creation logic
- [ ] Test price calculation
- [ ] Test booking code generation
- [ ] Test authorization checks

#### 11.2 Backend Integration Tests ⏳
- [ ] `backend/tests/booking_flow_test.rs`
  - [ ] Full booking flow with database
  - [ ] Test race conditions (concurrent bookings)
  - [ ] Test payment deadline enforcement

#### 11.3 Frontend Unit Tests ⏳
- [ ] Test components render correctly
- [ ] Test API client functions
- [ ] Test stores

#### 11.4 End-to-End Tests ⏳
- [ ] User registers → books session → pays → confirmed
- [ ] Organizer creates session → users book
- [ ] Booking timeout → slot returned

---

## 📊 Progress Summary

### Overall Completion: 0/54 tasks (0%)

**Completed**: 0
**In Progress**: 0
**Pending**: 54

### By Category:
- 🔐 Authentication: 0/4 (0%)
- 📅 Sessions: 0/5 (0%)
- 🎫 Bookings: 0/6 (0%)
- 💳 Payments: 0/5 (0%)
- ⏰ Background Jobs: 0/3 (0%)
- 🎨 Frontend Setup: 0/4 (0%)
- 👤 Frontend Auth: 0/5 (0%)
- 📋 Frontend Sessions: 0/5 (0%)
- 🎟️ Frontend Bookings: 0/5 (0%)
- 🏠 Frontend Layout: 0/3 (0%)
- ✅ Testing: 0/4 (0%)

---

## 🎯 Current Sprint Focus

**Sprint 1** (Target: 3 days)
- [ ] Complete Authentication (backend + frontend)
- [ ] Complete Session Management (backend + frontend)

**Sprint 2** (Target: 3 days)
- [ ] Complete Booking System (backend + frontend)
- [ ] Complete Stripe Integration

**Sprint 3** (Target: 2 days)
- [ ] Background Jobs
- [ ] Testing
- [ ] Bug fixes and polish

---

## 📝 Notes

- Update this file after completing each task
- Mark completed tasks with ✅
- Note any blockers or issues encountered
- Track time spent on major features

---

**Last Updated**: December 20, 2024
