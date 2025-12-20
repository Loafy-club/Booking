<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { loadStripe, type Stripe, type StripeElements } from '@stripe/stripe-js';
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
	import { Alert, AlertDescription } from '$lib/components/ui/alert';
	import type { Booking } from '$lib/types';

	let bookingId = $derived($page.params.id);
	let booking = $state<Booking | null>(null);
	let loading = $state(true);
	let error = $state<string | null>(null);
	let paymentLoading = $state(false);
	let paymentError = $state<string | null>(null);

	let stripe: Stripe | null = null;
	let elements: StripeElements | null = null;
	let paymentElement: HTMLDivElement | null = null;

	const stripePublishableKey = import.meta.env.VITE_STRIPE_PUBLISHABLE_KEY || '';

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

			if (!booking.cancelled_at && booking.payment_status === 'pending' && booking.payment_method === 'stripe') {
				await initializeStripe();
			} else if (booking.payment_status === 'confirmed') {
				error = 'This booking has already been paid';
			} else if (booking.cancelled_at) {
				error = 'This booking has been cancelled';
			} else if (booking.payment_method !== 'stripe') {
				error = 'This payment method is not supported yet';
			}
		} catch (err: any) {
			error = err.response?.data?.message || err.message || 'Failed to load booking';
		} finally {
			loading = false;
		}
	}

	async function initializeStripe() {
		try {
			stripe = await loadStripe(stripePublishableKey);

			if (!stripe) {
				throw new Error('Failed to load Stripe');
			}

			const response = await api.payments.createIntent(bookingId!);
			const { client_secret } = response.data;

			elements = stripe.elements({ clientSecret: client_secret });

			const payment = elements.create('payment');

			if (paymentElement) {
				payment.mount(paymentElement);
			}
		} catch (err: any) {
			error = err.response?.data?.message || err.message || 'Failed to initialize payment';
		}
	}

	async function handleSubmit(e: Event) {
		e.preventDefault();

		if (!stripe || !elements) {
			paymentError = 'Payment system not initialized';
			return;
		}

		paymentLoading = true;
		paymentError = null;

		try {
			const { error: submitError } = await elements.submit();

			if (submitError) {
				throw new Error(submitError.message);
			}

			const { error: confirmError } = await stripe.confirmPayment({
				elements,
				confirmParams: {
					return_url: `${window.location.origin}/bookings/${bookingId}?payment=success`
				}
			});

			if (confirmError) {
				throw new Error(confirmError.message);
			}
		} catch (err: any) {
			paymentError = err.message || 'Payment failed';
			paymentLoading = false;
		}
	}
</script>

<svelte:head>
	<title>Payment - Booking {booking?.booking_code || ''} - Loafy Club</title>
	<link rel="preconnect" href="https://fonts.googleapis.com">
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous">
	<link href="https://fonts.googleapis.com/css2?family=Baloo+2:wght@400;500;600;700;800&display=swap" rel="stylesheet">
</svelte:head>

<PageBackground variant="subtle">
	<Navigation />

	<div class="mx-auto max-w-4xl px-4 py-8 sm:px-6 lg:px-8">
		{#if loading}
			<LoadingSpinner text="Loading payment..." />
		{:else if error && !booking}
			<ErrorState
				message={error}
				onRetry={() => goto('/bookings')}
				retryText="Back to Bookings"
			/>
		{:else if error}
			<AnimatedContainer animation="fade-up">
				<BackButton href="/bookings" label="Back to Bookings" />
			</AnimatedContainer>
			<AnimatedContainer animation="fade-up" delay={100}>
				<ErrorState
					message={error}
					onRetry={() => goto(`/bookings/${bookingId}`)}
					retryText="View Booking"
				/>
			</AnimatedContainer>
		{:else if booking}
			<AnimatedContainer animation="fade-up">
				<BackButton href={`/bookings/${bookingId}`} label="Back to Booking" />
			</AnimatedContainer>

			<div class="grid gap-6 lg:grid-cols-3">
				<div class="lg:col-span-2">
					<AnimatedContainer animation="fade-up" delay={100}>
						<GlassCard>
							<h1 class="text-2xl font-bold text-gray-800" style="font-family: 'Baloo 2', sans-serif;">
								Complete Payment
							</h1>
							<p class="mt-2 text-sm text-gray-600">
								Booking {booking.booking_code}
							</p>

							{#if booking.payment_deadline}
								<Alert class="mt-4 bg-yellow-50 border-yellow-200">
									<AlertDescription class="text-yellow-800">
										Complete payment by {formatDate(booking.payment_deadline)} to confirm your booking
									</AlertDescription>
								</Alert>
							{/if}

							<form onsubmit={handleSubmit} class="mt-8">
								<div bind:this={paymentElement} class="rounded-xl border-2 border-gray-200 p-4 bg-white"></div>

								{#if paymentError}
									<Alert class="mt-4" variant="destructive">
										<AlertDescription>{paymentError}</AlertDescription>
									</Alert>
								{/if}

								<Button
									type="submit"
									size="lg"
									class="mt-6 w-full bg-gradient-to-r from-orange-500 to-pink-500 border-0 py-6 text-lg"
									disabled={paymentLoading || !stripe || !elements}
								>
									{paymentLoading ? 'Processing...' : `Pay ${formatCurrency(booking.price_paid_vnd + booking.guest_price_paid_vnd)}`}
								</Button>

								<p class="mt-4 text-center text-xs text-gray-500">
									Payments are securely processed by Stripe
								</p>
							</form>
						</GlassCard>
					</AnimatedContainer>
				</div>

				<div class="lg:col-span-1">
					<AnimatedContainer animation="fade-up" delay={200}>
						<GlassCard class="sticky top-20">
							<h2 class="text-lg font-bold text-gray-800" style="font-family: 'Baloo 2', sans-serif;">
								Order Summary
							</h2>

							<div class="mt-4 space-y-3">
								<div class="p-3 bg-gray-50 rounded-xl">
									<p class="text-xs text-gray-500">Booking Code</p>
									<p class="text-sm font-medium text-gray-800">{booking.booking_code}</p>
								</div>

								<div class="p-3 bg-gray-50 rounded-xl">
									<p class="text-xs text-gray-500">Number of People</p>
									<p class="text-sm font-medium text-gray-800">
										{1 + booking.guest_count}
									</p>
								</div>

								<div class="border-t border-gray-200 pt-3">
									<div class="flex justify-between text-sm">
										<span class="text-gray-600">Your slot</span>
										<span class="font-medium text-gray-800">{formatCurrency(booking.price_paid_vnd)}</span>
									</div>
									{#if booking.guest_count > 0}
										<div class="mt-2 flex justify-between text-sm">
											<span class="text-gray-600">Guest slots ({booking.guest_count})</span>
											<span class="font-medium text-gray-800">{formatCurrency(booking.guest_price_paid_vnd)}</span>
										</div>
									{/if}
								</div>

								<div class="border-t border-gray-200 pt-3">
									<div class="flex justify-between">
										<span class="text-base font-semibold text-gray-800">Total</span>
										<span class="text-xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-orange-500 to-pink-500">
											{formatCurrency(booking.price_paid_vnd + booking.guest_price_paid_vnd)}
										</span>
									</div>
								</div>
							</div>

							<div class="mt-6 p-4 bg-gray-50 rounded-xl">
								<div class="flex items-center gap-2 mb-2">
									<svg class="h-4 w-4 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
									</svg>
									<h3 class="text-xs font-medium text-gray-700">Secure Payment</h3>
								</div>
								<p class="text-xs text-gray-600">
									Your payment information is encrypted and processed securely by Stripe. We never store your card details.
								</p>
							</div>
						</GlassCard>
					</AnimatedContainer>
				</div>
			</div>
		{/if}
	</div>
</PageBackground>
