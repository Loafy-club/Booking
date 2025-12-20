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
	import { StatusBadge } from '$lib/components/ui/status-badge';
	import { AnimatedContainer } from '$lib/components/ui/animated-container';
	import { Alert, AlertDescription, AlertTitle } from '$lib/components/ui/alert';
	import type { Booking } from '$lib/types';

	let bookingId = $derived($page.params.id);
	let booking = $state<Booking | null>(null);
	let loading = $state(true);
	let error = $state<string | null>(null);
	let cancelLoading = $state(false);

	onMount(async () => {
		if (!requireAuth()) return;
		await loadBooking();
	});

	async function loadBooking() {
		loading = true;
		error = null;

		try {
			const response = await api.bookings.get(bookingId!);
			booking = response.data;
		} catch (err: any) {
			error = err.response?.data?.message || err.message || 'Failed to load booking';
		} finally {
			loading = false;
		}
	}

	async function handleCancel() {
		if (!booking) return;

		if (!confirm(`Are you sure you want to cancel booking ${booking.booking_code}?`)) {
			return;
		}

		cancelLoading = true;

		try {
			await api.bookings.cancel(bookingId!);
			await loadBooking();
		} catch (err: any) {
			alert(err.response?.data?.message || err.message || 'Failed to cancel booking');
		} finally {
			cancelLoading = false;
		}
	}

	function isPending(): boolean {
		return booking?.payment_status === 'pending' && !booking?.cancelled_at;
	}

	function canCancel(): boolean {
		return !booking?.cancelled_at && booking?.payment_status !== 'confirmed';
	}
</script>

<svelte:head>
	<title>Booking {booking?.booking_code || ''} - Loafy Club</title>
	<link rel="preconnect" href="https://fonts.googleapis.com">
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous">
	<link href="https://fonts.googleapis.com/css2?family=Baloo+2:wght@400;500;600;700;800&display=swap" rel="stylesheet">
</svelte:head>

<PageBackground variant="subtle">
	<Navigation />

	<div class="mx-auto max-w-4xl px-4 py-8 sm:px-6 lg:px-8">
		{#if loading}
			<LoadingSpinner text="Loading booking..." />
		{:else if error && !booking}
			<ErrorState
				message={error}
				onRetry={() => goto('/bookings')}
				retryText="Back to Bookings"
			/>
		{:else if booking}
			<AnimatedContainer animation="fade-up">
				<BackButton href="/bookings" label="Back to Bookings" />
			</AnimatedContainer>

			<AnimatedContainer animation="fade-up" delay={100}>
				<GlassCard>
					<div class="mb-6 flex items-center justify-between flex-wrap gap-4">
						<div>
							<h1 class="text-3xl font-bold text-gray-800" style="font-family: 'Baloo 2', sans-serif;">
								Booking {booking.booking_code}
							</h1>
							<p class="mt-1 text-sm text-gray-500">Created on {formatDate(booking.created_at)}</p>
						</div>

						<div class="flex gap-2">
							<StatusBadge status={booking.payment_status} variant="booking" />
							{#if booking.cancelled_at}
								<StatusBadge status="cancelled" variant="booking" />
							{/if}
						</div>
					</div>

					<div class="space-y-6">
						<div class="grid gap-6 md:grid-cols-2">
							<div class="p-4 bg-gray-50 rounded-xl">
								<h3 class="text-sm font-medium text-gray-500">Payment Status</h3>
								<p class="mt-1 text-lg font-semibold capitalize text-gray-800">
									{booking.payment_status}
								</p>
							</div>

							<div class="p-4 bg-gray-50 rounded-xl">
								<h3 class="text-sm font-medium text-gray-500">Payment Method</h3>
								<p class="mt-1 text-lg font-semibold text-gray-800">
									{booking.payment_method === 'stripe' ? 'Card Payment' : 'QR Payment'}
								</p>
							</div>

							<div class="p-4 bg-gray-50 rounded-xl">
								<h3 class="text-sm font-medium text-gray-500">Number of People</h3>
								<p class="mt-1 text-lg font-semibold text-gray-800">
									{1 + booking.guest_count}
									<span class="text-sm font-normal text-gray-600">
										(You + {booking.guest_count} guest{booking.guest_count !== 1 ? 's' : ''})
									</span>
								</p>
							</div>

							<div class="p-4 bg-gray-50 rounded-xl">
								<h3 class="text-sm font-medium text-gray-500">Total Amount</h3>
								<p class="mt-1 text-2xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-orange-500 to-pink-500">
									{formatCurrency(booking.price_paid_vnd + booking.guest_price_paid_vnd)}
								</p>
							</div>
						</div>

						{#if booking.price_paid_vnd > 0 || booking.guest_price_paid_vnd > 0}
							<div class="border-t border-gray-200 pt-6">
								<h3 class="text-sm font-medium text-gray-500 mb-3">Price Breakdown</h3>
								<div class="space-y-2 p-4 bg-gray-50 rounded-xl">
									<div class="flex justify-between text-sm">
										<span class="text-gray-600">Your slot</span>
										<span class="font-medium text-gray-800">{formatCurrency(booking.price_paid_vnd)}</span>
									</div>
									{#if booking.guest_count > 0}
										<div class="flex justify-between text-sm">
											<span class="text-gray-600">Guest slots ({booking.guest_count})</span>
											<span class="font-medium text-gray-800">{formatCurrency(booking.guest_price_paid_vnd)}</span>
										</div>
									{/if}
								</div>
							</div>
						{/if}

						{#if isPending() && booking.payment_deadline}
							<Alert class="bg-yellow-50 border-yellow-200">
								<svg class="h-5 w-5 text-yellow-600" fill="currentColor" viewBox="0 0 20 20">
									<path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
								</svg>
								<AlertTitle class="text-yellow-800">Payment Required</AlertTitle>
								<AlertDescription class="text-yellow-700">
									Please complete payment by {formatDate(booking.payment_deadline)} to confirm your booking.
								</AlertDescription>
							</Alert>
						{/if}

						{#if booking.cancelled_at}
							<Alert class="bg-red-50 border-red-200">
								<svg class="h-5 w-5 text-red-600" fill="currentColor" viewBox="0 0 20 20">
									<path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
								</svg>
								<AlertTitle class="text-red-800">Booking Cancelled</AlertTitle>
								<AlertDescription class="text-red-700">
									This booking was cancelled on {formatDate(booking.cancelled_at)}.
								</AlertDescription>
							</Alert>
						{/if}

						{#if booking.stripe_payment_intent_id}
							<div class="border-t border-gray-200 pt-6">
								<h3 class="text-sm font-medium text-gray-500">Payment Details</h3>
								<p class="mt-1 text-sm text-gray-600 font-mono bg-gray-50 p-3 rounded-lg">
									{booking.stripe_payment_intent_id}
								</p>
							</div>
						{/if}
					</div>

					<div class="mt-8 flex gap-4 flex-wrap border-t border-gray-200 pt-6">
						{#if isPending()}
							<Button
								size="lg"
								class="bg-gradient-to-r from-orange-500 to-pink-500 border-0"
								onclick={() => goto(`/bookings/${booking?.id}/payment`)}
							>
								Complete Payment
							</Button>
						{/if}

						{#if canCancel()}
							<Button
								variant="destructive"
								size="lg"
								disabled={cancelLoading}
								onclick={handleCancel}
							>
								{cancelLoading ? 'Cancelling...' : 'Cancel Booking'}
							</Button>
						{/if}

						<Button variant="outline" size="lg" onclick={() => goto('/bookings')}>
							Back to Bookings
						</Button>
					</div>
				</GlassCard>
			</AnimatedContainer>
		{/if}
	</div>
</PageBackground>
