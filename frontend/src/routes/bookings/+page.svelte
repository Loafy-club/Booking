<script lang="ts">
	import { onMount } from 'svelte';
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
	import { SectionHeader } from '$lib/components/ui/section-header';
	import { LoadingSpinner } from '$lib/components/ui/loading-spinner';
	import { ErrorState } from '$lib/components/ui/error-state';
	import { EmptyState } from '$lib/components/ui/empty-state';
	import { StatusBadge } from '$lib/components/ui/status-badge';
	import { AnimatedContainer } from '$lib/components/ui/animated-container';
	import { Alert, AlertDescription } from '$lib/components/ui/alert';
	import type { Booking } from '$lib/types';

	const t = useTranslation();

	let bookings = $state<Booking[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);

	onMount(async () => {
		if (!requireAuth()) return;
		await loadBookings();
	});

	async function loadBookings() {
		loading = true;
		error = null;

		try {
			const response = await api.bookings.list();
			bookings = response.data;
		} catch (err: unknown) {
			error = extractErrorMessage(err, 'Failed to load bookings');
		} finally {
			loading = false;
		}
	}

	async function handleCancelBooking(bookingId: string, bookingCode: string) {
		if (!confirm(t('bookings.confirmCancel', { code: bookingCode }))) {
			return;
		}

		try {
			await api.bookings.cancel(bookingId);
			await loadBookings();
		} catch (err: unknown) {
			alert(extractErrorMessage(err, t('bookings.cancelError')));
		}
	}

	function getPaymentMethodLabel(method: string): string {
		return method === 'stripe' ? t('bookings.cardPayment') : t('bookings.qrPayment');
	}
</script>

<svelte:head>
	<title>My Bookings - Loafy Club</title>
</svelte:head>

<PageBackground variant="subtle">
	<Navigation />

	<div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
		<AnimatedContainer animation="fade-up">
			<SectionHeader
				title={t('bookings.title')}
				subtitle={t('bookings.subtitle')}
			/>
		</AnimatedContainer>

		{#if loading}
			<LoadingSpinner text={t('bookings.loadingText')} />
		{:else if error}
			<ErrorState message={error} onRetry={loadBookings} />
		{:else if bookings.length === 0}
			<EmptyState
				title={t('bookings.noBookings')}
				description={t('bookings.noBookingsDesc')}
				actionText={t('bookings.browseSessions')}
				onAction={() => goto('/sessions')}
			/>
		{:else}
			<div class="space-y-4">
				{#each bookings as booking, i}
					<AnimatedContainer animation="fade-up" delay={100 + i * 50} trigger="scroll">
						<GlassCard>
							<div class="flex items-start justify-between">
								<div class="flex-1">
									<div class="flex items-center gap-3 flex-wrap">
									<h3 class="text-xl font-bold text-gray-800 font-display">
										{t('bookings.bookingCode')} {booking.booking_code}
									</h3>
										<StatusBadge status={booking.payment_status} variant="booking" />
										{#if booking.cancelled_at}
											<StatusBadge status="cancelled" variant="booking" />
										{/if}
									</div>

									<div class="mt-4 grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
										<div>
											<p class="text-xs text-gray-500">{t('bookings.bookedOn')}</p>
											<p class="text-sm font-medium text-gray-800">
												{formatDate(booking.created_at)}
											</p>
										</div>

										<div>
											<p class="text-xs text-gray-500">{t('bookings.paymentMethod')}</p>
											<p class="text-sm font-medium text-gray-800">
												{getPaymentMethodLabel(booking.payment_method)}
											</p>
										</div>

										<div>
											<p class="text-xs text-gray-500">{t('bookings.numberOfPeople')}</p>
											<p class="text-sm font-medium text-gray-800">
												{1 + booking.guest_count}
											</p>
										</div>

								<div>
									<p class="text-xs text-gray-500">{t('bookings.totalAmount')}</p>
								<p class="text-sm font-medium text-gradient-primary">
									{formatCurrency(getBookingTotal(booking))}
								</p>
								</div>
							</div>

							{#if isBookingPending(booking) && booking.payment_deadline}
										<Alert class="mt-4 bg-yellow-50 border-yellow-200">
											<AlertDescription class="text-yellow-800">
												{t('bookings.paymentRequired', { date: formatDate(booking.payment_deadline) })}
											</AlertDescription>
										</Alert>
									{/if}

									{#if booking.cancelled_at}
										<Alert class="mt-4 bg-red-50 border-red-200">
											<AlertDescription class="text-red-800">
												{t('bookings.cancelledOn', { date: formatDate(booking.cancelled_at) })}
											</AlertDescription>
										</Alert>
									{/if}
								</div>
							</div>

							<div class="mt-6 pt-4 border-t border-gray-200 flex gap-3 flex-wrap">
								<Button
									variant="outline"
									size="sm"
									onclick={() => goto(`/bookings/${booking.id}`)}
								>
									{t('bookings.viewDetails')}
								</Button>

								{#if isBookingPending(booking)}
									<Button
										variant="gradient"
										size="sm"
										onclick={() => goto(`/bookings/${booking.id}/payment`)}
									>
										{t('bookings.completePayment')}
									</Button>
								{/if}

								{#if canCancelBooking(booking)}
									<Button
										variant="destructive"
										size="sm"
										onclick={() => handleCancelBooking(booking.id, booking.booking_code)}
									>
										{t('bookings.cancel')}
									</Button>
								{/if}
							</div>
						</GlassCard>
					</AnimatedContainer>
				{/each}
			</div>
		{/if}
	</div>
</PageBackground>
