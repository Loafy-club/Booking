<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { api } from '$lib/api/client';
	import {
		formatCurrency,
		formatDate,
		isBookingPending,
		canCancelBooking,
		getBookingTotal,
		extractErrorMessage
	} from '$lib/utils';
	import { requireAuth } from '$lib/guards/auth';
	import { useTranslation } from '$lib/i18n/index.svelte';
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
	import { Card } from '$lib/components/ui/card';
	import { AlertTriangle, XCircle } from 'lucide-svelte';
	import type { Booking } from '$lib/types';

	const t = useTranslation();

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
		} catch (err: unknown) {
			error = extractErrorMessage(err, 'Failed to load booking');
		} finally {
			loading = false;
		}
	}

	async function handleCancel() {
		if (!booking) return;

		if (!confirm(t('bookings.confirmCancel', { code: booking.booking_code }))) {
			return;
		}

		cancelLoading = true;

		try {
			await api.bookings.cancel(bookingId!);
			await loadBooking();
		} catch (err: unknown) {
			alert(extractErrorMessage(err, t('bookings.cancelError')));
		} finally {
			cancelLoading = false;
		}
	}
</script>

<svelte:head>
	<title>{t('bookings.bookingCode')} {booking?.booking_code || ''} - Loafy Club</title>
</svelte:head>

<PageBackground variant="subtle">
	<Navigation />

	<div class="mx-auto max-w-4xl px-4 py-8 sm:px-6 lg:px-8">
		{#if loading}
			<LoadingSpinner text={t('bookings.loadingBooking')} />
		{:else if error && !booking}
			<ErrorState
				message={error}
				onRetry={() => goto('/bookings')}
				retryText={t('bookings.backToBookings')}
			/>
		{:else if booking}
			<AnimatedContainer animation="fade-up">
				<BackButton href="/bookings" label={t('bookings.backToBookings')} />
			</AnimatedContainer>

			<AnimatedContainer animation="fade-up" delay={100}>
				<GlassCard>
					<div class="mb-6 flex items-center justify-between flex-wrap gap-4">
						<div>
						<h1 class="text-3xl font-bold text-gray-800 font-display">
							{t('bookings.bookingCode')} {booking.booking_code}
						</h1>
							<p class="mt-1 text-sm text-gray-500">{t('bookings.createdOn', { date: formatDate(booking.created_at) })}</p>
						</div>

						<div class="flex gap-2">
							<StatusBadge status={t(`bookings.status.${booking.payment_status}`)} statusKey={booking.payment_status} variant="booking" />
							{#if booking.cancelled_at}
								<StatusBadge status={t('bookings.status.cancelled')} statusKey="cancelled" variant="booking" />
							{/if}
						</div>
					</div>

					<div class="space-y-6">
						<div class="grid gap-6 md:grid-cols-2">
							<Card variant="info">
								<h3 class="text-sm font-medium text-gray-500">{t('bookings.paymentStatus')}</h3>
								<p class="mt-1 text-lg font-semibold text-gray-800">
									{t(`bookings.status.${booking.payment_status}`)}
								</p>
							</Card>

							<Card variant="info">
								<h3 class="text-sm font-medium text-gray-500">{t('bookings.paymentMethod')}</h3>
								<p class="mt-1 text-lg font-semibold text-gray-800">
									{booking.payment_method === 'stripe' ? t('bookings.cardPayment') : t('bookings.qrPayment')}
								</p>
							</Card>

							<Card variant="info">
								<h3 class="text-sm font-medium text-gray-500">{t('bookings.numberOfPeople')}</h3>
								<p class="mt-1 text-lg font-semibold text-gray-800">
									{1 + booking.guest_count}
									<span class="text-sm font-normal text-gray-600">
										({booking.guest_count !== 1 ? t('bookings.youPlusGuestsPlural', { count: booking.guest_count }) : t('bookings.youPlusGuests', { count: booking.guest_count })})
									</span>
								</p>
							</Card>

							<Card variant="info">
								<h3 class="text-sm font-medium text-gray-500">{t('bookings.totalAmount')}</h3>
							<p class="mt-1 text-2xl font-bold text-gradient-primary">
								{formatCurrency(getBookingTotal(booking))}
							</p>
							</Card>
						</div>

						{#if getBookingTotal(booking) > 0}
							<div class="border-t border-gray-200 pt-6">
								<h3 class="text-sm font-medium text-gray-500 mb-3">{t('bookings.priceBreakdown')}</h3>
								<Card variant="info" class="space-y-2">
									<div class="flex justify-between text-sm">
										<span class="text-gray-600">{t('bookings.yourSlot')}</span>
										<span class="font-medium text-gray-800">{formatCurrency(booking.price_paid_vnd)}</span>
									</div>
									{#if booking.guest_count > 0}
										<div class="flex justify-between text-sm">
											<span class="text-gray-600">{t('bookings.guestSlots', { count: booking.guest_count })}</span>
											<span class="font-medium text-gray-800">{formatCurrency(booking.guest_price_paid_vnd)}</span>
										</div>
									{/if}
								</Card>
							</div>
						{/if}

						{#if isBookingPending(booking) && booking.payment_deadline}
							<Alert class="bg-yellow-50 border-yellow-200">
								<AlertTriangle class="h-5 w-5 text-yellow-600" />
								<AlertTitle class="text-yellow-800">{t('bookings.paymentRequiredTitle')}</AlertTitle>
								<AlertDescription class="text-yellow-700">
									{t('bookings.paymentRequiredDesc', { date: formatDate(booking.payment_deadline) })}
								</AlertDescription>
							</Alert>
						{/if}

						{#if booking.cancelled_at}
							<Alert class="bg-red-50 border-red-200">
								<XCircle class="h-5 w-5 text-red-600" />
								<AlertTitle class="text-red-800">{t('bookings.bookingCancelledTitle')}</AlertTitle>
								<AlertDescription class="text-red-700">
									{t('bookings.bookingCancelledDesc', { date: formatDate(booking.cancelled_at) })}
								</AlertDescription>
							</Alert>
						{/if}

						{#if booking.stripe_payment_intent_id}
							<div class="border-t border-gray-200 pt-6">
								<h3 class="text-sm font-medium text-gray-500">{t('bookings.paymentDetails')}</h3>
								<p class="mt-1 text-sm text-gray-600 font-mono bg-gray-50 p-3 rounded-lg">
									{booking.stripe_payment_intent_id}
								</p>
							</div>
						{/if}
					</div>

					<div class="mt-8 flex gap-4 flex-wrap border-t border-gray-200 pt-6">
						{#if isBookingPending(booking)}
							<Button
								variant="gradient"
								size="lg"
								onclick={() => goto(`/bookings/${booking?.id}/payment`)}
							>
								{t('bookings.completePayment')}
							</Button>
						{/if}

						{#if canCancelBooking(booking)}
							<Button
								variant="destructive"
								size="lg"
								disabled={cancelLoading}
								onclick={handleCancel}
							>
								{cancelLoading ? t('bookings.cancelling') : t('bookings.cancelBooking')}
							</Button>
						{/if}

						<Button variant="outline" size="lg" onclick={() => goto('/bookings')}>
							{t('bookings.backToBookings')}
						</Button>
					</div>
				</GlassCard>
			</AnimatedContainer>
		{/if}
	</div>
</PageBackground>
