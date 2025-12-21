<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { loadStripe, type Stripe, type StripeElements } from '@stripe/stripe-js';
	import { api } from '$lib/api/client';
	import { formatCurrency, formatDate, getBookingTotal, extractErrorMessage } from '$lib/utils';
	import { requireAuth } from '$lib/guards/auth';
	import { useTranslation } from '$lib/i18n/index.svelte';
	import Navigation from '$lib/components/Navigation.svelte';
	import { Button } from '$lib/components/ui/button';
	import { GlassCard } from '$lib/components/ui/glass-card';
	import { PageBackground } from '$lib/components/ui/page-background';
	import { BackButton } from '$lib/components/ui/back-button';
	import { LoadingSpinner } from '$lib/components/ui/loading-spinner';
	import { ErrorState } from '$lib/components/ui/error-state';
	import { AnimatedContainer } from '$lib/components/ui/animated-container';
	import { Alert, AlertDescription } from '$lib/components/ui/alert';
	import { Card } from '$lib/components/ui/card';
	import { ShieldCheck } from 'lucide-svelte';
	import type { Booking } from '$lib/types';

	const t = useTranslation();

	let bookingId = $derived($page.params.id);
	let booking = $state<Booking | null>(null);
	let loading = $state(true);
	let error = $state<string | null>(null);
	let paymentLoading = $state(false);
	let paymentError = $state<string | null>(null);

	let stripe = $state<Stripe | null>(null);
	let elements = $state<StripeElements | null>(null);
	let paymentElement = $state<HTMLDivElement | null>(null);

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
			const loadedBooking = response.data;
			booking = loadedBooking;

			if (!loadedBooking.cancelled_at && loadedBooking.payment_status === 'pending' && loadedBooking.payment_method === 'stripe') {
				await initializeStripe();
			} else if (loadedBooking.payment_status === 'confirmed') {
				error = t('payment.alreadyPaid');
			} else if (loadedBooking.cancelled_at) {
				error = t('payment.bookingCancelled');
			} else if (loadedBooking.payment_method !== 'stripe') {
				error = t('payment.methodNotSupported');
			}
		} catch (err: unknown) {
			error = extractErrorMessage(err, t('payment.failedToLoad'));
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
		} catch (err: unknown) {
			error = extractErrorMessage(err, t('payment.failedToInit'));
		}
	}

	async function handleSubmit(e: Event) {
		e.preventDefault();

		if (!stripe || !elements) {
			paymentError = t('payment.paymentNotInit');
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
		} catch (err: unknown) {
			paymentError = extractErrorMessage(err, t('payment.paymentFailed'));
			paymentLoading = false;
		}
	}
</script>

<svelte:head>
	<title>{t('payment.pageTitle')} - {t('bookings.bookingCode')} {booking?.booking_code || ''} - Loafy Club</title>
</svelte:head>

<PageBackground variant="subtle">
	<Navigation />

	<div class="mx-auto max-w-4xl px-4 py-8 sm:px-6 lg:px-8">
		{#if loading}
			<LoadingSpinner text={t('payment.loadingPayment')} />
		{:else if error && !booking}
			<ErrorState
				message={error}
				onRetry={() => goto('/bookings')}
				retryText={t('bookings.backToBookings')}
			/>
		{:else if error}
			<AnimatedContainer animation="fade-up">
				<BackButton href="/bookings" label={t('bookings.backToBookings')} />
			</AnimatedContainer>
			<AnimatedContainer animation="fade-up" delay={100}>
				<ErrorState
					message={error}
					onRetry={() => goto(`/bookings/${bookingId}`)}
					retryText={t('payment.viewBooking')}
				/>
			</AnimatedContainer>
		{:else if booking}
			<AnimatedContainer animation="fade-up">
				<BackButton href={`/bookings/${bookingId}`} label={t('bookings.backToBooking')} />
			</AnimatedContainer>

			<div class="grid gap-6 lg:grid-cols-3">
				<div class="lg:col-span-2">
					<AnimatedContainer animation="fade-up" delay={100}>
						<GlassCard>
							<h1 class="text-2xl font-bold text-gray-800 font-display">
								{t('payment.completePayment')}
							</h1>
							<p class="mt-2 text-sm text-gray-600">
								{t('bookings.bookingCode')} {booking.booking_code}
							</p>

							{#if booking.payment_deadline}
								<Alert class="mt-4 bg-yellow-50 border-yellow-200">
									<AlertDescription class="text-yellow-800">
										{t('payment.completeBy', { date: formatDate(booking.payment_deadline) })}
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
									variant="gradient"
									size="lg"
									class="mt-6 w-full py-6 text-lg"
									disabled={paymentLoading || !stripe || !elements}
								>
									{paymentLoading ? t('payment.processing') : t('payment.pay', { amount: formatCurrency(getBookingTotal(booking)) })}
								</Button>

								<p class="mt-4 text-center text-xs text-gray-500">
									{t('payment.securedByStripe')}
								</p>
							</form>
						</GlassCard>
					</AnimatedContainer>
				</div>

				<div class="lg:col-span-1">
					<AnimatedContainer animation="fade-up" delay={200}>
						<GlassCard class="sticky top-20">
							<h2 class="text-lg font-bold text-gray-800 font-display">
								{t('payment.orderSummary')}
							</h2>

							<div class="mt-4 space-y-3">
								<Card variant="infoSm">
									<p class="text-xs text-gray-500">{t('bookings.bookingCode')}</p>
									<p class="text-sm font-medium text-gray-800">{booking.booking_code}</p>
								</Card>

								<Card variant="infoSm">
									<p class="text-xs text-gray-500">{t('bookings.numberOfPeople')}</p>
									<p class="text-sm font-medium text-gray-800">
										{1 + booking.guest_count}
									</p>
								</Card>

								<div class="border-t border-gray-200 pt-3">
									<div class="flex justify-between text-sm">
										<span class="text-gray-600">{t('bookings.yourSlot')}</span>
										<span class="font-medium text-gray-800">{formatCurrency(booking.price_paid_vnd)}</span>
									</div>
									{#if booking.guest_count > 0}
										<div class="mt-2 flex justify-between text-sm">
											<span class="text-gray-600">{t('bookings.guestSlots', { count: booking.guest_count })}</span>
											<span class="font-medium text-gray-800">{formatCurrency(booking.guest_price_paid_vnd)}</span>
										</div>
									{/if}
								</div>

								<div class="border-t border-gray-200 pt-3">
									<div class="flex justify-between">
										<span class="text-base font-semibold text-gray-800">{t('payment.total')}</span>
										<span class="text-xl font-bold text-gradient-primary">
											{formatCurrency(getBookingTotal(booking))}
										</span>
									</div>
								</div>
							</div>

							<Card variant="info" class="mt-6">
								<div class="flex items-center gap-2 mb-2">
									<ShieldCheck class="h-4 w-4 text-green-600" />
									<h3 class="text-xs font-medium text-gray-700">{t('payment.securePayment')}</h3>
								</div>
								<p class="text-xs text-gray-600">
									{t('payment.securePaymentDesc')}
								</p>
							</Card>
						</GlassCard>
					</AnimatedContainer>
				</div>
			</div>
		{/if}
	</div>
</PageBackground>
