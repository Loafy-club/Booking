<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { api } from '$lib/api/client';
	import { authStore } from '$lib/stores/auth.svelte';
	import { formatCurrency, formatDate } from '$lib/utils';
	import { requireAuth } from '$lib/guards/auth';
	import Navigation from '$lib/components/Navigation.svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
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

			const booking = response.data;

			// Redirect to payment page
			goto(`/bookings/${booking.id}/payment`);
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
</svelte:head>

<Navigation />

<div class="mx-auto max-w-4xl px-4 py-8 sm:px-6 lg:px-8">
	{#if loading}
		<div class="flex justify-center py-12">
			<div class="h-12 w-12 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
		</div>
	{:else if error && !session}
		<div class="rounded-lg bg-destructive/10 p-6 text-center">
			<p class="text-destructive">{error}</p>
			<Button class="mt-4" onclick={() => goto('/sessions')}>Back to Sessions</Button>
		</div>
	{:else if session}
		<div class="mb-6">
			<Button variant="ghost" onclick={() => goto('/sessions')}>
				← Back to Sessions
			</Button>
		</div>

		<div class="grid gap-6 lg:grid-cols-3">
			<div class="lg:col-span-2">
				<Card.Root class="p-6">
					<h1 class="text-3xl font-bold text-gray-900">{session.title}</h1>

					{#if session.description}
						<p class="mt-4 text-gray-700">{session.description}</p>
					{/if}

					<div class="mt-6 space-y-4">
						<div class="flex items-start">
							<svg class="mr-3 h-6 w-6 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"
								/>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"
								/>
							</svg>
							<div>
								<p class="font-medium text-gray-900">Location</p>
								<p class="text-gray-600">{session.location}</p>
							</div>
						</div>

						<div class="flex items-start">
							<svg class="mr-3 h-6 w-6 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
								/>
							</svg>
							<div>
								<p class="font-medium text-gray-900">Date & Time</p>
								<p class="text-gray-600">{formatDate(session.start_time, 'long')}</p>
								<p class="text-sm text-gray-500">Ends at {formatDate(session.end_time, 'long')}</p>
							</div>
						</div>

						<div class="flex items-start">
							<svg class="mr-3 h-6 w-6 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"
								/>
							</svg>
							<div>
								<p class="font-medium text-gray-900">Availability</p>
								<p class="text-gray-600">
									{session.available_slots} of {session.max_slots} spots available
								</p>
							</div>
						</div>

						<div class="flex items-start">
							<svg class="mr-3 h-6 w-6 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
								/>
							</svg>
							<div>
								<p class="font-medium text-gray-900">Price</p>
								<p class="text-2xl font-bold text-primary">{formatCurrency(session.price_vnd)}</p>
								<p class="text-sm text-gray-500">per person</p>
							</div>
						</div>
					</div>
				</Card.Root>
			</div>

			<div class="lg:col-span-1">
				<Card.Root class="sticky top-4 p-6">
					<h2 class="text-xl font-semibold text-gray-900">Book Your Spot</h2>

					{#if !canBook()}
						<div class="mt-4 rounded-md bg-yellow-50 p-4">
							<p class="text-sm text-yellow-800">{getSessionStatusText()}</p>
						</div>
					{:else}
						<div class="mt-6 space-y-4">
							<div>
								<label for="guest-count" class="block text-sm font-medium text-gray-700">Number of Guests</label>
								<select
									id="guest-count"
									bind:value={guestCount}
									class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary"
								>
									<option value={0}>Just me</option>
									{#each Array(Math.min(3, session.available_slots - 1)) as _, i}
										<option value={i + 1}>{i + 1} guest{i > 0 ? 's' : ''}</option>
									{/each}
								</select>
								<p class="mt-1 text-xs text-gray-500">Maximum 3 guests per booking</p>
							</div>

							<fieldset>
								<legend class="block text-sm font-medium text-gray-700">Payment Method</legend>
								<div class="mt-2 space-y-2">
									<label class="flex items-center">
										<input
											type="radio"
											bind:group={paymentMethod}
											value="stripe"
											class="h-4 w-4 text-primary focus:ring-primary"
										/>
										<span class="ml-2 text-sm text-gray-700">Credit/Debit Card (Stripe)</span>
									</label>
									<label class="flex items-center">
										<input
											type="radio"
											bind:group={paymentMethod}
											value="qr"
											class="h-4 w-4 text-primary focus:ring-primary"
											disabled
										/>
										<span class="ml-2 text-sm text-gray-500">QR Payment (Coming Soon)</span>
									</label>
								</div>
							</fieldset>

							<div class="border-t pt-4">
								<div class="flex justify-between text-sm text-gray-600">
									<span>Price per person</span>
									<span>{formatCurrency(session.price_vnd)}</span>
								</div>
								<div class="mt-2 flex justify-between text-sm text-gray-600">
									<span>Number of people</span>
									<span>{1 + guestCount}</span>
								</div>
								<div class="mt-2 flex justify-between text-lg font-bold text-gray-900">
									<span>Total</span>
									<span>{formatCurrency(getTotalPrice())}</span>
								</div>
							</div>

							{#if error}
								<div class="rounded-md bg-destructive/10 p-3">
									<p class="text-sm text-destructive">{error}</p>
								</div>
							{/if}

							<Button
								class="w-full"
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
				</Card.Root>
			</div>
		</div>
	{/if}
</div>
