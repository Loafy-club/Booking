<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { api } from '$lib/api/client';
	import { formatCurrency, formatDate } from '$lib/utils';
	import { requireAuth } from '$lib/guards/auth';
	import Navigation from '$lib/components/Navigation.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Card from '$lib/components/ui/Card.svelte';
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

	function getStatusColor(status: string): string {
		switch (status) {
			case 'pending':
				return 'bg-yellow-100 text-yellow-800';
			case 'confirmed':
				return 'bg-green-100 text-green-800';
			case 'failed':
				return 'bg-red-100 text-red-800';
			default:
				return 'bg-gray-100 text-gray-800';
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
</svelte:head>

<Navigation />

<div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
	<div class="mb-8">
		<h1 class="text-3xl font-bold text-gray-900">My Bookings</h1>
		<p class="mt-2 text-gray-600">View and manage your session bookings</p>
	</div>

	{#if loading}
		<div class="flex justify-center py-12">
			<div class="h-12 w-12 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
		</div>
	{:else if error}
		<div class="rounded-lg bg-destructive/10 p-6 text-center">
			<p class="text-destructive">{error}</p>
			<Button class="mt-4" onclick={loadBookings}>Try Again</Button>
		</div>
	{:else if bookings.length === 0}
		<div class="rounded-lg bg-gray-50 p-12 text-center">
			<p class="text-lg text-gray-600">No bookings yet</p>
			<p class="mt-2 text-sm text-gray-500">Browse sessions to make your first booking</p>
			<Button class="mt-4" onclick={() => goto('/sessions')}>Browse Sessions</Button>
		</div>
	{:else}
		<div class="space-y-4">
			{#each bookings as booking}
				<Card class="overflow-hidden">
					<div class="p-6">
						<div class="flex items-start justify-between">
							<div class="flex-1">
								<div class="flex items-center gap-3">
									<h3 class="text-xl font-semibold text-gray-900">
										Booking {booking.booking_code}
									</h3>
									<span class={`rounded-full px-2 py-1 text-xs font-medium ${getStatusColor(booking.payment_status)}`}>
										{booking.payment_status}
									</span>
									{#if booking.cancelled_at}
										<span class="rounded-full bg-red-100 px-2 py-1 text-xs font-medium text-red-800">
											Cancelled
										</span>
									{/if}
								</div>

								<div class="mt-4 grid gap-3 sm:grid-cols-2 lg:grid-cols-4">
									<div>
										<p class="text-xs text-gray-500">Booked On</p>
										<p class="text-sm font-medium text-gray-900">
											{formatDate(booking.created_at)}
										</p>
									</div>

									<div>
										<p class="text-xs text-gray-500">Payment Method</p>
										<p class="text-sm font-medium text-gray-900">
											{getPaymentMethodLabel(booking.payment_method)}
										</p>
									</div>

									<div>
										<p class="text-xs text-gray-500">Number of People</p>
										<p class="text-sm font-medium text-gray-900">
											{1 + booking.guest_count}
										</p>
									</div>

									<div>
										<p class="text-xs text-gray-500">Total Amount</p>
										<p class="text-sm font-medium text-primary">
											{formatCurrency(booking.price_paid_vnd + booking.guest_price_paid_vnd)}
										</p>
									</div>
								</div>

								{#if isPending(booking) && booking.payment_deadline}
									<div class="mt-4 rounded-md bg-yellow-50 p-3">
										<p class="text-sm text-yellow-800">
											Payment required by {formatDate(booking.payment_deadline)}
										</p>
									</div>
								{/if}

								{#if booking.cancelled_at}
									<div class="mt-4 rounded-md bg-red-50 p-3">
										<p class="text-sm text-red-800">
											Cancelled on {formatDate(booking.cancelled_at)}
										</p>
									</div>
								{/if}
							</div>
						</div>
					</div>

					<div class="border-t bg-gray-50 px-6 py-4 flex gap-3">
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
				</Card>
			{/each}
		</div>
	{/if}
</div>
