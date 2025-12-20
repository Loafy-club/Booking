<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { api } from '$lib/api/client';
	import { formatCurrency, formatDate } from '$lib/utils';
	import { requireAuth } from '$lib/guards/auth';
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
		} catch (err: any) {
			error = err.response?.data?.message || err.message || 'Failed to load bookings';
		} finally {
			loading = false;
		}
	}

	async function handleCancelBooking(bookingId: string, bookingCode: string) {
		if (!confirm(`Are you sure you want to cancel booking ${bookingCode}?`)) {
			return;
		}

		try {
			await api.bookings.cancel(bookingId);
			await loadBookings();
		} catch (err: any) {
			alert(err.response?.data?.message || err.message || 'Failed to cancel booking');
		}
	}

	function getPaymentMethodLabel(method: string): string {
		return method === 'stripe' ? 'Card Payment' : 'QR Payment';
	}

	function isPending(booking: Booking): boolean {
		return booking.payment_status === 'pending' && !booking.cancelled_at;
	}

	function canCancel(booking: Booking): boolean {
		return !booking.cancelled_at && booking.payment_status !== 'confirmed';
	}
</script>

<svelte:head>
	<title>My Bookings - Loafy Club</title>
	<link rel="preconnect" href="https://fonts.googleapis.com">
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous">
	<link href="https://fonts.googleapis.com/css2?family=Baloo+2:wght@400;500;600;700;800&display=swap" rel="stylesheet">
</svelte:head>

<PageBackground variant="subtle">
	<Navigation />

	<div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
		<AnimatedContainer animation="fade-up">
			<SectionHeader
				title="My Bookings"
				subtitle="View and manage your session bookings"
			/>
		</AnimatedContainer>

		{#if loading}
			<LoadingSpinner text="Loading bookings..." />
		{:else if error}
			<ErrorState message={error} onRetry={loadBookings} />
		{:else if bookings.length === 0}
			<EmptyState
				title="No Bookings Yet"
				description="Browse sessions to make your first booking"
				actionText="Browse Sessions"
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
										<h3 class="text-xl font-bold text-gray-800" style="font-family: 'Baloo 2', sans-serif;">
											Booking {booking.booking_code}
										</h3>
										<StatusBadge status={booking.payment_status} variant="booking" />
										{#if booking.cancelled_at}
											<StatusBadge status="cancelled" variant="booking" />
										{/if}
									</div>

									<div class="mt-4 grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
										<div>
											<p class="text-xs text-gray-500">Booked On</p>
											<p class="text-sm font-medium text-gray-800">
												{formatDate(booking.created_at)}
											</p>
										</div>

										<div>
											<p class="text-xs text-gray-500">Payment Method</p>
											<p class="text-sm font-medium text-gray-800">
												{getPaymentMethodLabel(booking.payment_method)}
											</p>
										</div>

										<div>
											<p class="text-xs text-gray-500">Number of People</p>
											<p class="text-sm font-medium text-gray-800">
												{1 + booking.guest_count}
											</p>
										</div>

										<div>
											<p class="text-xs text-gray-500">Total Amount</p>
											<p class="text-sm font-medium text-transparent bg-clip-text bg-gradient-to-r from-orange-500 to-pink-500">
												{formatCurrency(booking.price_paid_vnd + booking.guest_price_paid_vnd)}
											</p>
										</div>
									</div>

									{#if isPending(booking) && booking.payment_deadline}
										<Alert class="mt-4 bg-yellow-50 border-yellow-200">
											<AlertDescription class="text-yellow-800">
												Payment required by {formatDate(booking.payment_deadline)}
											</AlertDescription>
										</Alert>
									{/if}

									{#if booking.cancelled_at}
										<Alert class="mt-4 bg-red-50 border-red-200">
											<AlertDescription class="text-red-800">
												Cancelled on {formatDate(booking.cancelled_at)}
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
									View Details
								</Button>

								{#if isPending(booking)}
									<Button
										size="sm"
										class="bg-gradient-to-r from-orange-500 to-pink-500 border-0"
										onclick={() => goto(`/bookings/${booking.id}/payment`)}
									>
										Complete Payment
									</Button>
								{/if}

								{#if canCancel(booking)}
									<Button
										variant="destructive"
										size="sm"
										onclick={() => handleCancelBooking(booking.id, booking.booking_code)}
									>
										Cancel
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
