<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { api } from '$lib/api/client';
	import { formatCurrency, formatDate } from '$lib/utils';
	import { requireAuth } from '$lib/guards/auth';
	import Navigation from '$lib/components/Navigation.svelte';
	import { Button } from '$lib/components/ui/button';
	import { GlassCard } from '$lib/components/ui/glass-card';
	import { PageBackground } from '$lib/components/ui/page-background';
	import { BackButton } from '$lib/components/ui/back-button';
	import { LoadingSpinner } from '$lib/components/ui/loading-spinner';
	import { ErrorState } from '$lib/components/ui/error-state';
	import { AnimatedContainer } from '$lib/components/ui/animated-container';
	import { Label } from '$lib/components/ui/label';
	import { Alert, AlertDescription } from '$lib/components/ui/alert';
	import type { Session } from '$lib/types';

	let sessionId = $derived($page.params.id);
	let session = $state<Session | null>(null);
	let loading = $state(true);
	let error = $state<string | null>(null);
	let bookingLoading = $state(false);
	let guestCount = $state(0);
	let paymentMethod = $state<'stripe' | 'qr'>('stripe');

	onMount(async () => {
		if (!requireAuth()) return;
		await loadSession();
	});

	async function loadSession() {
		loading = true;
		error = null;

		try {
			const response = await api.sessions.get(sessionId);
			session = response.data;
		} catch (err: any) {
			error = err.response?.data?.message || err.message || 'Failed to load session';
		} finally {
			loading = false;
		}
	}

	async function handleBooking() {
		if (!session) return;

		const totalSlots = 1 + guestCount;
		if (totalSlots > session.available_slots) {
			error = `Only ${session.available_slots} spots available`;
			return;
		}

		bookingLoading = true;
		error = null;

		try {
			const response = await api.bookings.create({
				session_id: sessionId,
				guest_count: guestCount,
				payment_method: paymentMethod
			});

			goto(`/bookings/${response.data.id}/payment`);
		} catch (err: any) {
			error = err.response?.data?.message || err.message || 'Failed to create booking';
			bookingLoading = false;
		}
	}

	function canBook(): boolean {
		if (!session) return false;
		if (session.status !== 'published') return false;
		if (session.available_slots === 0) return false;

		const now = new Date();
		const startTime = new Date(session.start_time);

		return now < startTime;
	}

	function getTotalPrice(): number {
		if (!session) return 0;
		return session.price_vnd * (1 + guestCount);
	}

	function getSessionStatusText(): string {
		if (!session) return '';

		const now = new Date();
		const startTime = new Date(session.start_time);
		const endTime = new Date(session.end_time);

		if (session.status !== 'published') {
			return 'This session is not available for booking';
		}

		if (session.available_slots === 0) {
			return 'This session is fully booked';
		}

		if (now >= endTime) {
			return 'This session has ended';
		}

		if (now >= startTime) {
			return 'This session is currently in progress';
		}

		return '';
	}
</script>

<svelte:head>
	<title>{session?.title || 'Session'} - Loafy Club</title>
	<link rel="preconnect" href="https://fonts.googleapis.com">
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous">
	<link href="https://fonts.googleapis.com/css2?family=Baloo+2:wght@400;500;600;700;800&display=swap" rel="stylesheet">
</svelte:head>

<PageBackground variant="subtle">
	<Navigation />

	<div class="mx-auto max-w-4xl px-4 py-8 sm:px-6 lg:px-8">
		{#if loading}
			<LoadingSpinner text="Loading session..." />
		{:else if error && !session}
			<ErrorState
				message={error}
				onRetry={() => goto('/sessions')}
				retryText="Back to Sessions"
			/>
		{:else if session}
			<AnimatedContainer animation="fade-up">
				<BackButton href="/sessions" label="Back to Sessions" />
			</AnimatedContainer>

			<div class="grid gap-6 lg:grid-cols-3">
				<div class="lg:col-span-2">
					<AnimatedContainer animation="fade-up" delay={100}>
						<GlassCard>
							<h1 class="text-3xl font-bold text-gray-800" style="font-family: 'Baloo 2', sans-serif;">
								{session.title}
							</h1>

							{#if session.description}
								<p class="mt-4 text-gray-600">{session.description}</p>
							{/if}

							<div class="mt-6 space-y-4">
								<div class="flex items-start">
									<svg class="mr-3 h-6 w-6 text-orange-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
									</svg>
									<div>
										<p class="font-medium text-gray-800">Location</p>
										<p class="text-gray-600">{session.location}</p>
									</div>
								</div>

								<div class="flex items-start">
									<svg class="mr-3 h-6 w-6 text-orange-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
									</svg>
									<div>
										<p class="font-medium text-gray-800">Date & Time</p>
										<p class="text-gray-600">{formatDate(session.start_time, 'long')}</p>
										<p class="text-sm text-gray-500">Ends at {formatDate(session.end_time, 'long')}</p>
									</div>
								</div>

								<div class="flex items-start">
									<svg class="mr-3 h-6 w-6 text-orange-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
									</svg>
									<div>
										<p class="font-medium text-gray-800">Availability</p>
										<p class="text-gray-600">
											{session.available_slots} of {session.max_slots} spots available
										</p>
									</div>
								</div>

								<div class="flex items-start">
									<svg class="mr-3 h-6 w-6 text-orange-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
									</svg>
									<div>
										<p class="font-medium text-gray-800">Price</p>
										<p class="text-2xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-orange-500 to-pink-500">
											{formatCurrency(session.price_vnd)}
										</p>
										<p class="text-sm text-gray-500">per person</p>
									</div>
								</div>
							</div>
						</GlassCard>
					</AnimatedContainer>
				</div>

				<div class="lg:col-span-1">
					<AnimatedContainer animation="fade-up" delay={200}>
						<GlassCard class="sticky top-20">
							<h2 class="text-xl font-bold text-gray-800" style="font-family: 'Baloo 2', sans-serif;">
								Book Your Spot
							</h2>

							{#if !canBook()}
								<Alert class="mt-4 bg-yellow-50 border-yellow-200">
									<AlertDescription class="text-yellow-800">
										{getSessionStatusText()}
									</AlertDescription>
								</Alert>
							{:else}
								<div class="mt-6 space-y-4">
									<div>
										<Label for="guest-count">Number of Guests</Label>
										<select
											id="guest-count"
											bind:value={guestCount}
											class="mt-1 block w-full rounded-xl border-2 border-gray-200 px-4 py-3 focus:border-orange-400 focus:outline-none focus:ring-2 focus:ring-orange-200 transition-colors"
										>
											<option value={0}>Just me</option>
											{#each Array(Math.min(3, session.available_slots - 1)) as _, i}
												<option value={i + 1}>{i + 1} guest{i > 0 ? 's' : ''}</option>
											{/each}
										</select>
										<p class="mt-1 text-xs text-gray-500">Maximum 3 guests per booking</p>
									</div>

									<fieldset>
										<Label>Payment Method</Label>
										<div class="mt-2 space-y-2">
											<label class="flex items-center gap-3 p-3 rounded-xl border-2 border-gray-200 cursor-pointer hover:border-orange-300 transition-colors {paymentMethod === 'stripe' ? 'border-orange-400 bg-orange-50' : ''}">
												<input
													type="radio"
													bind:group={paymentMethod}
													value="stripe"
													class="h-4 w-4 text-orange-500 focus:ring-orange-400"
												/>
												<span class="text-sm text-gray-700">Credit/Debit Card (Stripe)</span>
											</label>
											<label class="flex items-center gap-3 p-3 rounded-xl border-2 border-gray-200 cursor-not-allowed opacity-50">
												<input
													type="radio"
													bind:group={paymentMethod}
													value="qr"
													class="h-4 w-4 text-orange-500 focus:ring-orange-400"
													disabled
												/>
												<span class="text-sm text-gray-500">QR Payment (Coming Soon)</span>
											</label>
										</div>
									</fieldset>

									<div class="border-t border-gray-200 pt-4">
										<div class="flex justify-between text-sm text-gray-600">
											<span>Price per person</span>
											<span>{formatCurrency(session.price_vnd)}</span>
										</div>
										<div class="mt-2 flex justify-between text-sm text-gray-600">
											<span>Number of people</span>
											<span>{1 + guestCount}</span>
										</div>
										<div class="mt-2 flex justify-between text-lg font-bold text-gray-800">
											<span>Total</span>
											<span class="text-transparent bg-clip-text bg-gradient-to-r from-orange-500 to-pink-500">
												{formatCurrency(getTotalPrice())}
											</span>
										</div>
									</div>

									{#if error}
										<Alert variant="destructive">
											<AlertDescription>{error}</AlertDescription>
										</Alert>
									{/if}

									<Button
										class="w-full bg-gradient-to-r from-orange-500 to-pink-500 border-0 py-6 text-lg"
										size="lg"
										disabled={bookingLoading}
										onclick={handleBooking}
									>
										{bookingLoading ? 'Creating Booking...' : 'Book Now'}
									</Button>

									<p class="text-xs text-center text-gray-500">
										You'll have 30 minutes to complete payment
									</p>
								</div>
							{/if}
						</GlassCard>
					</AnimatedContainer>
				</div>
			</div>
		{/if}
	</div>
</PageBackground>
