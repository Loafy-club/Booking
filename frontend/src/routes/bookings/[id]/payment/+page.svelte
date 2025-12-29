<script lang="ts">
	import { onMount, tick } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { loadStripe, type Stripe, type StripeElements } from '@stripe/stripe-js';
	import { api } from '$lib/api/client';
	import { formatCurrency, formatDate, extractErrorMessage } from '$lib/utils';
	import { requireAuth } from '$lib/guards/auth';
	import { useTranslation } from '$lib/i18n/index.svelte';
	import Navigation from '$lib/components/Navigation.svelte';
	import { Button } from '$lib/components/ui/button';
	import { PageBackground } from '$lib/components/ui/page-background';
	import { BookingDetailSkeleton } from '$lib/components/ui/skeleton';
	import * as Empty from '$lib/components/ui/empty';
	import { AnimatedContainer } from '$lib/components/ui/animated-container';
	import { Alert, AlertDescription } from '$lib/components/ui/alert';
	import { Card } from '$lib/components/ui/card';
	import { Countdown } from '$lib/components/ui/countdown';
	import { ShieldCheck, ArrowLeft, AlertCircle, Clock } from 'lucide-svelte';
	import type { Booking } from '$lib/types';

	const t = useTranslation();

	let bookingId = $derived($page.params.id);
	let booking = $state<Booking | null>(null);
	let loading = $state(true);
	let showSkeleton = $state(false);
	let error = $state<string | null>(null);
	let paymentLoading = $state(false);
	let paymentError = $state<string | null>(null);

	let stripe = $state<Stripe | null>(null);
	let elements = $state<StripeElements | null>(null);
	let paymentElement = $state<HTMLDivElement | null>(null);

	const stripePublishableKey = import.meta.env.VITE_STRIPE_PUBLISHABLE_KEY || '';

	onMount(async () => {
		if (!(await requireAuth())) return;

		const skeletonTimer = setTimeout(() => {
			if (loading) showSkeleton = true;
		}, 200);

		await loadBooking();
		clearTimeout(skeletonTimer);
	});

	let shouldInitStripe = $state(false);

	async function loadBooking() {
		loading = true;
		error = null;

		try {
			const response = await api.bookings.get(bookingId!);
			const loadedBooking = response.data;
			booking = loadedBooking;

			if (!loadedBooking.cancelled_at && loadedBooking.payment_status === 'pending' && loadedBooking.payment_method === 'stripe') {
				shouldInitStripe = true;
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

	// Initialize Stripe when the payment element is available
	$effect(() => {
		if (shouldInitStripe && paymentElement && !stripe) {
			initializeStripe();
		}
	});

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

			// Wait for DOM to update after booking is set
			await tick();

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

	<main class="mx-auto max-w-4xl px-4 py-8 sm:px-6 lg:px-8">
		{#if loading && showSkeleton}
			<BookingDetailSkeleton />
		{:else if loading}
			<!-- Brief loading -->
		{:else if error && !booking}
			<AnimatedContainer animation="fade-up" delay={100}>
				<Card variant="glass" class="mx-auto max-w-md">
					<Empty.Root>
						<Empty.Header>
							<Empty.Media variant="icon">
								<AlertCircle class="size-5" />
							</Empty.Media>
							<Empty.Title>{t('common.error')}</Empty.Title>
							<Empty.Description>{error}</Empty.Description>
						</Empty.Header>
						<Empty.Content>
							<Button onclick={() => goto('/bookings')}>{t('bookings.backToBookings')}</Button>
						</Empty.Content>
					</Empty.Root>
				</Card>
			</AnimatedContainer>
		{:else if error}
			<AnimatedContainer animation="fade-up">
				<div class="mb-6">
					<Button variant="ghost" onclick={() => goto('/bookings')}>
						<ArrowLeft class="size-4 mr-2" />
						{t('bookings.backToBookings')}
					</Button>
				</div>
			</AnimatedContainer>
			<AnimatedContainer animation="fade-up" delay={100}>
				<Card variant="glass" class="mx-auto max-w-md">
					<Empty.Root>
						<Empty.Header>
							<Empty.Media variant="icon">
								<AlertCircle class="size-5" />
							</Empty.Media>
							<Empty.Title>{t('common.error')}</Empty.Title>
							<Empty.Description>{error}</Empty.Description>
						</Empty.Header>
						<Empty.Content>
							<Button onclick={() => goto(`/bookings/${bookingId}`)}>{t('payment.viewBooking')}</Button>
						</Empty.Content>
					</Empty.Root>
				</Card>
			</AnimatedContainer>
		{:else if booking}
			<AnimatedContainer animation="fade-up">
				<div class="mb-6">
					<Button variant="ghost" onclick={() => goto(`/bookings/${bookingId}`)}>
						<ArrowLeft class="size-4 mr-2" />
						{t('bookings.backToBooking')}
					</Button>
				</div>
			</AnimatedContainer>

			<div class="grid gap-6 lg:grid-cols-3">
				<div class="lg:col-span-2">
					<AnimatedContainer animation="fade-up" delay={100}>
						<Card variant="glass">
							<h1 class="text-2xl font-bold text-foreground font-display">
								{t('payment.completePayment')}
							</h1>
							<p class="mt-2 text-sm text-muted-foreground">
								{t('bookings.bookingCode')} {booking.booking_code}
							</p>

							{#if booking.payment_deadline}
								<Alert class="mt-4" variant="warning">
									<AlertDescription class="flex items-center gap-2">
										<Clock class="h-4 w-4 shrink-0" />
										{t('countdown.timeRemaining')}:
										<Countdown deadline={booking.payment_deadline} compact class="text-warning-foreground" />
									</AlertDescription>
								</Alert>
							{/if}

							<form onsubmit={handleSubmit} class="mt-8">
								<div bind:this={paymentElement}></div>

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
									{paymentLoading ? t('payment.processing') : t('payment.pay', { amount: formatCurrency(booking.total_paid_vnd) })}
								</Button>

								<p class="mt-4 text-center text-xs text-muted-foreground">
									{t('payment.securedByStripe')}
								</p>
							</form>
						</Card>
					</AnimatedContainer>
				</div>

				<div class="lg:col-span-1">
					<AnimatedContainer animation="fade-up" delay={200}>
						<Card variant="glass" class="sticky top-20">
							<h2 class="text-lg font-bold text-foreground font-display">
								{t('payment.orderSummary')}
							</h2>

							<div class="mt-4 space-y-3">
								<Card variant="infoSm">
									<p class="text-xs text-muted-foreground">{t('bookings.bookingCode')}</p>
									<p class="text-sm font-medium text-foreground">{booking.booking_code}</p>
								</Card>

								<Card variant="infoSm">
									<p class="text-xs text-muted-foreground">{t('bookings.numberOfPeople')}</p>
									<p class="text-sm font-medium text-foreground">
										{1 + booking.guest_count}
									</p>
								</Card>

								<!-- Price breakdown -->
								<div class="border-t border-border pt-3 space-y-2">
									<!-- Your slot section -->
									<div class="space-y-1">
										<div class="flex justify-between text-sm">
											<span class="text-muted-foreground">{t('bookings.yourSlot')}</span>
											{#if booking.discount_applied === 'ticket'}
												<span class="font-medium text-foreground line-through text-muted-foreground">
													{formatCurrency(booking.session_price_vnd)}
												</span>
											{:else if booking.discount_applied === 'out_of_ticket'}
												<span class="font-medium text-foreground line-through text-muted-foreground">
													{formatCurrency(booking.session_price_vnd)}
												</span>
											{:else}
												<span class="font-medium text-foreground">{formatCurrency(booking.session_price_vnd)}</span>
											{/if}
										</div>

										{#if booking.discount_applied === 'ticket'}
											<div class="flex justify-between text-sm">
												<span class="text-success-text flex items-center gap-1">
													<span class="inline-block w-2 h-2 rounded-full bg-success"></span>
													{t('pricing.ticketUsed')}
												</span>
												<span class="font-medium text-success-text">-{formatCurrency(booking.session_price_vnd)}</span>
											</div>
										{:else if booking.discount_applied === 'out_of_ticket'}
											<div class="flex justify-between text-sm">
												<span class="text-primary flex items-center gap-1">
													<span class="inline-block w-2 h-2 rounded-full bg-primary"></span>
													{t('pricing.subscriberDiscount')}
												</span>
												<span class="font-medium text-primary">-{formatCurrency(booking.session_price_vnd - booking.price_paid_vnd)}</span>
											</div>
										{/if}

										{#if booking.discount_applied !== 'none'}
											<div class="flex justify-between text-sm font-medium">
												<span class="text-muted-foreground">{t('pricing.subtotalUser')}</span>
												<span class="text-foreground">{formatCurrency(booking.price_paid_vnd)}</span>
											</div>
										{/if}
									</div>

									<!-- Guest slots section -->
									{#if booking.guest_count > 0}
										<div class="pt-2 space-y-1">
											<div class="flex justify-between text-sm">
												<span class="text-muted-foreground">
													{t('bookings.guestSlots', { count: booking.guest_count })}
												</span>
												<span class="font-medium text-foreground">
													{booking.guest_count} × {formatCurrency(booking.session_price_vnd)}
												</span>
											</div>
											<div class="flex justify-between text-sm font-medium">
												<span class="text-muted-foreground">{t('pricing.subtotalGuests')}</span>
												<span class="text-foreground">{formatCurrency(booking.guest_price_paid_vnd)}</span>
											</div>
										</div>
									{/if}
								</div>

								<div class="border-t border-border pt-3">
									<div class="flex justify-between">
										<span class="text-base font-semibold text-foreground">{t('payment.total')}</span>
										<span class="text-xl font-bold text-gradient-primary">
											{formatCurrency(booking.total_paid_vnd)}
										</span>
									</div>
								</div>
							</div>

							<Card variant="info" class="mt-6">
								<div class="flex items-center gap-2 mb-2">
									<ShieldCheck class="h-4 w-4 text-success-text" />
									<h3 class="text-xs font-medium text-foreground">{t('payment.securePayment')}</h3>
								</div>
								<p class="text-xs text-muted-foreground">
									{t('payment.securePaymentDesc')}
								</p>
							</Card>
						</Card>
					</AnimatedContainer>
				</div>
			</div>
		{/if}
	</main>
</PageBackground>
