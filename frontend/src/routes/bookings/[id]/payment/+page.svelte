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
	import * as Card from '$lib/components/ui/card';
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
			const response = await api.bookings.get(bookingId);
			booking = response.data;

			// Check if payment is needed
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
			// Load Stripe
			stripe = await loadStripe(stripePublishableKey);

			if (!stripe) {
				throw new Error('Failed to load Stripe');
			}

			// Create payment intent
			const response = await api.payments.createIntent(bookingId);
			const { client_secret } = response.data;

			// Create Stripe elements
			elements = stripe.elements({ clientSecret: client_secret });

			// Create and mount payment element
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
</svelte:head>

<Navigation />

<div class="mx-auto max-w-4xl px-4 py-8 sm:px-6 lg:px-8">
	{#if loading}
		<div class="flex justify-center py-12">
			<div class="h-12 w-12 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
		</div>
	{:else if error && !booking}
		<div class="rounded-lg bg-destructive/10 p-6 text-center">
			<p class="text-destructive">{error}</p>
			<Button class="mt-4" onclick={() => goto('/bookings')}>Back to Bookings</Button>
		</div>
	{:else if error}
		<div class="mb-6">
			<Button variant="ghost" onclick={() => goto('/bookings')}>
				← Back to Bookings
			</Button>
		</div>
		<div class="rounded-lg bg-destructive/10 p-6 text-center">
			<p class="text-destructive">{error}</p>
			<Button class="mt-4" onclick={() => goto(`/bookings/${bookingId}`)}>View Booking</Button>
		</div>
	{:else if booking}
		<div class="mb-6">
			<Button variant="ghost" onclick={() => goto(`/bookings/${bookingId}`)}>
				← Back to Booking
			</Button>
		</div>

		<div class="grid gap-6 lg:grid-cols-3">
			<div class="lg:col-span-2">
				<Card.Root class="p-6">
					<h1 class="text-2xl font-bold text-gray-900">Complete Payment</h1>
					<p class="mt-2 text-sm text-gray-600">
						Booking {booking.booking_code}
					</p>

					{#if booking.payment_deadline}
						<div class="mt-4 rounded-md bg-yellow-50 p-3">
							<p class="text-sm text-yellow-800">
								Complete payment by {formatDate(booking.payment_deadline)} to confirm your booking
							</p>
						</div>
					{/if}

					<form onsubmit={handleSubmit} class="mt-8">
						<div bind:this={paymentElement} class="rounded-md border p-4"></div>

						{#if paymentError}
							<div class="mt-4 rounded-md bg-destructive/10 p-4">
								<p class="text-sm text-destructive">{paymentError}</p>
							</div>
						{/if}

						<Button
							type="submit"
							size="lg"
							class="mt-6 w-full"
							disabled={paymentLoading || !stripe || !elements}
						>
							{paymentLoading ? 'Processing...' : `Pay ${formatCurrency(booking.price_paid_vnd + booking.guest_price_paid_vnd)}`}
						</Button>

						<p class="mt-4 text-center text-xs text-gray-500">
							Payments are securely processed by Stripe
						</p>
					</form>
				</Card.Root>
			</div>

			<div class="lg:col-span-1">
				<Card.Root class="sticky top-4 p-6">
					<h2 class="text-lg font-semibold text-gray-900">Order Summary</h2>

					<div class="mt-4 space-y-3">
						<div>
							<p class="text-xs text-gray-500">Booking Code</p>
							<p class="text-sm font-medium text-gray-900">{booking.booking_code}</p>
						</div>

						<div>
							<p class="text-xs text-gray-500">Number of People</p>
							<p class="text-sm font-medium text-gray-900">
								{1 + booking.guest_count}
							</p>
						</div>

						<div class="border-t pt-3">
							<div class="flex justify-between text-sm">
								<span class="text-gray-600">Your slot</span>
								<span class="font-medium text-gray-900">{formatCurrency(booking.price_paid_vnd)}</span>
							</div>
							{#if booking.guest_count > 0}
								<div class="mt-2 flex justify-between text-sm">
									<span class="text-gray-600">Guest slots ({booking.guest_count})</span>
									<span class="font-medium text-gray-900">{formatCurrency(booking.guest_price_paid_vnd)}</span>
								</div>
							{/if}
						</div>

						<div class="border-t pt-3">
							<div class="flex justify-between">
								<span class="text-base font-semibold text-gray-900">Total</span>
								<span class="text-xl font-bold text-primary">
									{formatCurrency(booking.price_paid_vnd + booking.guest_price_paid_vnd)}
								</span>
							</div>
						</div>
					</div>

					<div class="mt-6 rounded-md bg-gray-50 p-4">
						<h3 class="text-xs font-medium text-gray-700">Secure Payment</h3>
						<p class="mt-1 text-xs text-gray-600">
							Your payment information is encrypted and processed securely by Stripe. We never store your card details.
						</p>
					</div>
				</Card.Root>
			</div>
		</div>
	{/if}
</div>
