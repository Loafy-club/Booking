<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { api } from '$lib/api/client';
	import {
		formatCurrency,
		formatDate,
		formatDateOnly,
		formatTime,
		formatDuration,
		isBookingPending,
		canCancelBooking,
		extractErrorMessage
	} from '$lib/utils';
	import { requireAuth } from '$lib/guards/auth';
	import { useTranslation } from '$lib/i18n/index.svelte';
	import Navigation from '$lib/components/Navigation.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Card } from '$lib/components/ui/card';
	import { PageBackground } from '$lib/components/ui/page-background';
	import { BookingDetailSkeleton } from '$lib/components/ui/skeleton';
	import * as Empty from '$lib/components/ui/empty';
	import { Badge } from '$lib/components/ui/badge';
	import { AnimatedContainer } from '$lib/components/ui/animated-container';
	import { Alert, AlertDescription, AlertTitle } from '$lib/components/ui/alert';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { Countdown } from '$lib/components/ui/countdown';
	import { AlertTriangle, XCircle, ArrowLeft, AlertCircle, Clock, Calendar, MapPin } from 'lucide-svelte';
	import type { Booking } from '$lib/types';

	// Cancel dialog state
	let cancelDialogOpen = $state(false);
	let cancelError = $state<string | null>(null);

	const t = useTranslation();

	let bookingId = $derived($page.params.id);
	let booking = $state<Booking | null>(null);
	let loading = $state(true);
	let showSkeleton = $state(false);
	let error = $state<string | null>(null);
	let cancelLoading = $state(false);

	onMount(async () => {
		if (!(await requireAuth())) return;

		const skeletonTimer = setTimeout(() => {
			if (loading) showSkeleton = true;
		}, 200);

		await loadBooking();
		clearTimeout(skeletonTimer);
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

	function openCancelDialog() {
		cancelError = null;
		cancelDialogOpen = true;
	}

	async function confirmCancel() {
		if (!booking) return;

		cancelLoading = true;
		cancelError = null;

		try {
			await api.bookings.cancel(bookingId!);
			cancelDialogOpen = false;
			await loadBooking();
		} catch (err: unknown) {
			cancelError = extractErrorMessage(err, t('bookings.cancelError'));
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
		{:else if booking}
			<AnimatedContainer animation="fade-up">
				<div class="mb-6">
					<Button variant="ghost" onclick={() => goto('/bookings')}>
						<ArrowLeft class="size-4 mr-2" />
						{t('bookings.backToBookings')}
					</Button>
				</div>
			</AnimatedContainer>

			<AnimatedContainer animation="fade-up" delay={100}>
				<Card variant="glass">
					<div class="mb-6 flex items-center justify-between flex-wrap gap-4">
						<div>
						<h1 class="text-3xl font-bold font-display text-foreground">
							{t('bookings.bookingCode')} {booking.booking_code}
						</h1>
							<p class="mt-1 text-sm text-muted-foreground">{t('bookings.createdOn', { date: formatDate(booking.created_at) })}</p>
						</div>

						<div class="flex gap-2">
							<Badge variant={booking.payment_status as any}>{t(`bookings.status.${booking.payment_status}`)}</Badge>
						</div>
					</div>

					<!-- Session Info Section -->
					<Card variant="info" class="mb-6">
						<h3 class="text-lg font-semibold text-foreground mb-3">
							{booking.session_title}
						</h3>
						<div class="flex flex-wrap gap-4 text-sm">
							<div class="flex items-center gap-2 text-muted-foreground">
								<Calendar class="h-4 w-4" />
								<span>
									{formatDateOnly(booking.session_date)} • {formatTime(booking.session_time)}{#if booking.session_end_time} - {formatTime(booking.session_end_time)}{/if}
									{#if booking.session_end_time}
										{@const duration = formatDuration(booking.session_time, booking.session_end_time)}
										{#if duration}
											<span class="text-muted-foreground/70">({duration})</span>
										{/if}
									{/if}
								</span>
							</div>
							<div class="flex items-center gap-2 text-muted-foreground">
								<MapPin class="h-4 w-4" />
								<span>{booking.session_location}</span>
							</div>
						</div>
					</Card>

					<div class="space-y-6">
						<div class="grid gap-6 md:grid-cols-2">
							<Card variant="info">
								<h3 class="text-sm font-medium text-muted-foreground">{t('bookings.paymentStatus')}</h3>
								<p class="mt-1 text-lg font-semibold text-foreground">
									{t(`bookings.status.${booking.payment_status}`)}
								</p>
							</Card>

							<Card variant="info">
								<h3 class="text-sm font-medium text-muted-foreground">{t('bookings.paymentMethod')}</h3>
								<p class="mt-1 text-lg font-semibold text-foreground">
									{booking.payment_method === 'stripe' ? t('bookings.cardPayment') : t('bookings.qrPayment')}
								</p>
							</Card>

							<Card variant="info">
								<h3 class="text-sm font-medium text-muted-foreground">{t('bookings.numberOfPeople')}</h3>
								<p class="mt-1 text-lg font-semibold text-foreground">
									{1 + booking.guest_count}
									<span class="text-sm font-normal text-muted-foreground">
										({booking.guest_count !== 1 ? t('bookings.youPlusGuestsPlural', { count: booking.guest_count }) : t('bookings.youPlusGuests', { count: booking.guest_count })})
									</span>
								</p>
							</Card>

							<Card variant="info">
								<h3 class="text-sm font-medium text-muted-foreground">{t('bookings.totalAmount')}</h3>
							<p class="mt-1 text-2xl font-bold text-gradient-primary">
								{formatCurrency(booking.total_paid_vnd)}
							</p>
							</Card>
						</div>

						<!-- Price breakdown -->
						<div class="border-t border-border pt-6">
							<h3 class="text-sm font-medium text-muted-foreground mb-3">{t('bookings.priceBreakdown')}</h3>
							<Card variant="info" class="space-y-3">
								<!-- Your slot section -->
								<div class="space-y-1">
									<div class="flex justify-between text-sm">
										<span class="text-muted-foreground">{t('bookings.yourSlot')}</span>
										{#if booking.discount_applied === 'ticket'}
											<span class="font-medium line-through text-muted-foreground">
												{formatCurrency(booking.session_price_vnd)}
											</span>
										{:else if booking.discount_applied === 'out_of_ticket'}
											<span class="font-medium line-through text-muted-foreground">
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
									<div class="pt-2 border-t border-border/50 space-y-1">
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

								<!-- Total -->
								<div class="pt-2 border-t border-border">
									<div class="flex justify-between">
										<span class="font-semibold text-foreground">{t('payment.total')}</span>
										<span class="font-bold text-gradient-primary">{formatCurrency(booking.total_paid_vnd)}</span>
									</div>
								</div>
							</Card>
						</div>

						{#if isBookingPending(booking) && booking.payment_deadline}
							<Alert class="mt-4" variant="warning">
								<AlertDescription class="flex items-center gap-2">
									<Clock class="h-4 w-4 shrink-0" />
									{t('countdown.timeRemaining')}:
									<Countdown deadline={booking.payment_deadline} compact class="text-warning-foreground" />
								</AlertDescription>
							</Alert>
						{/if}

						{#if booking.cancelled_at}
							<Alert variant="destructive">
								<XCircle class="h-5 w-5" />
								<AlertTitle>{t('bookings.bookingCancelledTitle')}</AlertTitle>
								<AlertDescription>
									{t('bookings.bookingCancelledDesc', { date: formatDate(booking.cancelled_at) })}
								</AlertDescription>
							</Alert>
						{/if}

						{#if booking.stripe_payment_intent_id}
							<div class="border-t border-border pt-6">
								<h3 class="text-sm font-medium text-muted-foreground">{t('bookings.paymentDetails')}</h3>
								<p class="mt-1 text-sm text-muted-foreground font-mono bg-muted p-3 rounded-lg">
									{booking.stripe_payment_intent_id}
								</p>
							</div>
						{/if}
					</div>

					<div class="mt-8 flex gap-4 flex-wrap border-t border-border pt-6">
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
								onclick={openCancelDialog}
							>
								{t('bookings.cancelBooking')}
							</Button>
						{/if}

						<Button variant="outline" size="lg" onclick={() => goto('/bookings')}>
							{t('bookings.backToBookings')}
						</Button>
					</div>
				</Card>
			</AnimatedContainer>
		{/if}
	</main>
</PageBackground>

<!-- Cancel Booking Dialog -->
<AlertDialog.Root bind:open={cancelDialogOpen}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>{t('bookings.cancelDialogTitle')}</AlertDialog.Title>
			<AlertDialog.Description>
				{t('bookings.confirmCancel', { code: booking?.booking_code || '' })}
			</AlertDialog.Description>
		</AlertDialog.Header>

		{#if cancelError}
			<Alert variant="destructive">
				<AlertDescription>{cancelError}</AlertDescription>
			</Alert>
		{/if}

		<AlertDialog.Footer>
			<AlertDialog.Cancel disabled={cancelLoading}>
				{t('common.cancel')}
			</AlertDialog.Cancel>
			<Button
				variant="destructive"
				onclick={confirmCancel}
				disabled={cancelLoading}
			>
				{cancelLoading ? t('bookings.cancelling') : t('bookings.confirmCancelButton')}
			</Button>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
