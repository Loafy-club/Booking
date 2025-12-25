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
	import { Card } from '$lib/components/ui/card';
	import { PageBackground } from '$lib/components/ui/page-background';
	import { SectionHeader } from '$lib/components/ui/section-header';
	import { BookingCardSkeleton } from '$lib/components/ui/skeleton';
	import * as Empty from '$lib/components/ui/empty';
	import { Badge } from '$lib/components/ui/badge';
	import { AnimatedContainer } from '$lib/components/ui/animated-container';
	import { Alert, AlertDescription } from '$lib/components/ui/alert';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { Pagination } from '$lib/components/ui/pagination';
	import { Ticket } from 'lucide-svelte';
	import type { Booking, PageInfo } from '$lib/types';

	// Cancel booking dialog state
	let cancelDialogOpen = $state(false);
	let bookingToCancel = $state<Booking | null>(null);
	let cancelError = $state<string | null>(null);
	let cancelling = $state(false);

	const t = useTranslation();

	// Pagination state
	let currentPage = $state(1);
	let perPage = $state(5);

	// Data state
	let bookings = $state<Booking[]>([]);
	let pageInfo = $state<PageInfo | null>(null);
	let loading = $state(true);
	let showSkeleton = $state(false);
	let error = $state<string | null>(null);

	// Initialize from URL params
	function initFromUrl() {
		const params = $page.url.searchParams;
		currentPage = parseInt(params.get('page') || '1', 10);
	}

	// Update URL with current state
	function updateUrl() {
		const params = new URLSearchParams();
		if (currentPage > 1) params.set('page', currentPage.toString());

		const newUrl = params.toString() ? `?${params.toString()}` : '/bookings';
		goto(newUrl, { replaceState: true, keepFocus: true });
	}

	onMount(async () => {
		if (!(await requireAuth())) return;

		initFromUrl();

		const skeletonTimer = setTimeout(() => {
			if (loading) showSkeleton = true;
		}, 200);

		await loadBookings();
		clearTimeout(skeletonTimer);
	});

	async function loadBookings() {
		loading = true;
		error = null;

		try {
			const response = await api.bookings.list({
				page: currentPage,
				per_page: perPage
			});
			// Handle paginated response format
			if (response.data.data && response.data.page_info) {
				bookings = response.data.data;
				pageInfo = response.data.page_info;
			} else {
				// Fallback for old non-paginated response (array directly)
				bookings = Array.isArray(response.data) ? response.data : [];
				pageInfo = null;
			}
		} catch (err: unknown) {
			error = extractErrorMessage(err, 'Failed to load bookings');
		} finally {
			loading = false;
		}
	}

	// Handle pagination
	function handlePageChange(newPage: number) {
		currentPage = newPage;
		updateUrl();
		loadBookings();
	}

	function openCancelDialog(booking: Booking) {
		bookingToCancel = booking;
		cancelError = null;
		cancelDialogOpen = true;
	}

	async function confirmCancelBooking() {
		if (!bookingToCancel) return;

		cancelling = true;
		cancelError = null;

		try {
			await api.bookings.cancel(bookingToCancel.id);
			cancelDialogOpen = false;
			bookingToCancel = null;
			await loadBookings();
		} catch (err: unknown) {
			cancelError = extractErrorMessage(err, t('bookings.cancelError'));
		} finally {
			cancelling = false;
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

	<main class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
		<AnimatedContainer animation="fade-up">
			<SectionHeader
				title={t('bookings.title')}
				subtitle={t('bookings.subtitle')}
			/>
		</AnimatedContainer>

		{#if loading && showSkeleton}
			<div class="space-y-4">
				{#each Array(3) as _}
					<BookingCardSkeleton />
				{/each}
			</div>
		{:else if loading}
			<!-- Brief loading -->
		{:else if error}
			<AnimatedContainer animation="fade-up" delay={100}>
				<Card variant="glass" class="mx-auto max-w-md">
					<Alert variant="destructive">
						<AlertDescription>{error}</AlertDescription>
					</Alert>
					<Button class="mt-4 w-full" onclick={loadBookings}>{t('common.retry')}</Button>
				</Card>
			</AnimatedContainer>
		{:else if bookings.length === 0}
			<AnimatedContainer animation="fade-up" delay={100}>
				<Card variant="glass" class="mx-auto max-w-md">
					<Empty.Root>
						<Empty.Header>
							<Empty.Media variant="icon">
								<Ticket class="size-5" />
							</Empty.Media>
							<Empty.Title>{t('bookings.noBookings')}</Empty.Title>
							<Empty.Description>{t('bookings.noBookingsDesc')}</Empty.Description>
						</Empty.Header>
						<Empty.Content>
							<Button onclick={() => goto('/sessions')}>{t('bookings.browseSessions')}</Button>
						</Empty.Content>
					</Empty.Root>
				</Card>
			</AnimatedContainer>
		{:else}
			<div class="space-y-4">
				{#each bookings as booking, i}
					<AnimatedContainer animation="fade-up" delay={100 + i * 50} trigger="scroll">
						<Card variant="glass">
							<div class="flex items-start justify-between">
								<div class="flex-1">
									<div class="flex items-center gap-3 flex-wrap">
									<h3 class="text-xl font-bold font-display text-foreground">
										{t('bookings.bookingCode')} {booking.booking_code}
									</h3>
										<Badge variant={booking.payment_status as any}>{t(`bookings.status.${booking.payment_status}`)}</Badge>
										{#if booking.cancelled_at}
											<Badge variant="cancelled">{t('bookings.status.cancelled')}</Badge>
										{/if}
									</div>

									<div class="mt-4 grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
										<div>
											<p class="text-xs text-muted-foreground">{t('bookings.bookedOn')}</p>
											<p class="text-sm font-medium text-foreground">
												{formatDate(booking.created_at)}
											</p>
										</div>

										<div>
											<p class="text-xs text-muted-foreground">{t('bookings.paymentMethod')}</p>
											<p class="text-sm font-medium text-foreground">
												{getPaymentMethodLabel(booking.payment_method)}
											</p>
										</div>

										<div>
											<p class="text-xs text-muted-foreground">{t('bookings.numberOfPeople')}</p>
											<p class="text-sm font-medium text-foreground">
												{1 + booking.guest_count}
											</p>
										</div>

								<div>
									<p class="text-xs text-muted-foreground">{t('bookings.totalAmount')}</p>
								<p class="text-sm font-medium text-gradient-primary">
									{formatCurrency(getBookingTotal(booking))}
								</p>
								</div>
							</div>

							{#if isBookingPending(booking) && booking.payment_deadline}
										<Alert class="mt-4" variant="warning">
											<AlertDescription>
												{t('bookings.paymentRequired', { date: formatDate(booking.payment_deadline) })}
											</AlertDescription>
										</Alert>
									{/if}

									{#if booking.cancelled_at}
										<Alert class="mt-4" variant="destructive">
											<AlertDescription>
												{t('bookings.cancelledOn', { date: formatDate(booking.cancelled_at) })}
											</AlertDescription>
										</Alert>
									{/if}
								</div>
							</div>

							<div class="mt-6 pt-4 border-t border-border flex items-center gap-3 flex-wrap">
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
										onclick={() => openCancelDialog(booking)}
									>
										{t('bookings.cancel')}
									</Button>
								{/if}
							</div>
						</Card>
					</AnimatedContainer>
				{/each}
			</div>

			<!-- Pagination -->
			{#if pageInfo && pageInfo.total_pages > 1}
				<AnimatedContainer animation="fade-up" delay={150}>
					<div class="mt-6">
						<Pagination {pageInfo} onPageChange={handlePageChange} />
					</div>
				</AnimatedContainer>
			{:else if pageInfo}
				<AnimatedContainer animation="fade-up" delay={150}>
					<p class="mt-4 text-sm text-muted-foreground text-center">
						{t('bookings.totalCount', { count: pageInfo.total })}
					</p>
				</AnimatedContainer>
			{/if}
		{/if}
	</main>
</PageBackground>

<!-- Cancel Booking Dialog -->
<AlertDialog.Root bind:open={cancelDialogOpen}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>{t('bookings.cancelDialogTitle')}</AlertDialog.Title>
			<AlertDialog.Description>
				{t('bookings.confirmCancel', { code: bookingToCancel?.booking_code || '' })}
			</AlertDialog.Description>
		</AlertDialog.Header>

		{#if cancelError}
			<Alert variant="destructive">
				<AlertDescription>{cancelError}</AlertDescription>
			</Alert>
		{/if}

		<AlertDialog.Footer>
			<AlertDialog.Cancel disabled={cancelling}>
				{t('common.cancel')}
			</AlertDialog.Cancel>
			<Button
				variant="destructive"
				onclick={confirmCancelBooking}
				disabled={cancelling}
			>
				{cancelling ? t('bookings.cancelling') : t('bookings.confirmCancelButton')}
			</Button>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
