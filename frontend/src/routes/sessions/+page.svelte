<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { api } from '$lib/api/client';
	import { formatCurrency, formatDate } from '$lib/utils';
	import { requireAuth } from '$lib/guards/auth';
	import Navigation from '$lib/components/Navigation.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import type { Session } from '$lib/types';

	let sessions = $state<Session[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);
	let filter = $state<'upcoming' | 'all'>('upcoming');

	onMount(async () => {
		if (!requireAuth()) return;

		await loadSessions();
	});

	async function loadSessions() {
		loading = true;
		error = null;

		try {
			const now = new Date();
			const params: any = {};

			if (filter === 'upcoming') {
				params.start_date = now.toISOString();
				params.status = 'published';
			}

			const response = await api.sessions.list(params);
			sessions = response.data;
		} catch (err: any) {
			error = err.response?.data?.message || err.message || 'Failed to load sessions';
		} finally {
			loading = false;
		}
	}

	function getSessionStatus(session: Session): string {
		const now = new Date();
		const startTime = new Date(session.start_time);
		const endTime = new Date(session.end_time);

		if (session.status !== 'published') {
			return session.status;
		}

		if (session.available_slots === 0) {
			return 'full';
		}

		if (now < startTime) {
			return 'upcoming';
		}

		if (now >= startTime && now <= endTime) {
			return 'in_progress';
		}

		return 'completed';
	}

	function getStatusColor(status: string): string {
		switch (status) {
			case 'upcoming':
				return 'bg-green-100 text-green-800';
			case 'in_progress':
				return 'bg-blue-100 text-blue-800';
			case 'full':
				return 'bg-yellow-100 text-yellow-800';
			case 'completed':
				return 'bg-gray-100 text-gray-800';
			default:
				return 'bg-gray-100 text-gray-800';
		}
	}

	$effect(() => {
		if (filter) {
			loadSessions();
		}
	});
</script>

<svelte:head>
	<title>Sessions - Loafy Club</title>
</svelte:head>

<Navigation />

<div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
	<div class="mb-8 flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold text-gray-900">Pickleball Sessions</h1>
			<p class="mt-2 text-gray-600">Browse and book available sessions</p>
		</div>

		<div class="flex gap-2">
			<Button
				variant={filter === 'upcoming' ? 'default' : 'outline'}
				onclick={() => (filter = 'upcoming')}
			>
				Upcoming
			</Button>
			<Button
				variant={filter === 'all' ? 'default' : 'outline'}
				onclick={() => (filter = 'all')}
			>
				All Sessions
			</Button>
		</div>
	</div>

	{#if loading}
		<div class="flex justify-center py-12">
			<div class="h-12 w-12 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
		</div>
	{:else if error}
		<div class="rounded-lg bg-destructive/10 p-6 text-center">
			<p class="text-destructive">{error}</p>
			<Button class="mt-4" onclick={loadSessions}>Try Again</Button>
		</div>
	{:else if sessions.length === 0}
		<div class="rounded-lg bg-gray-50 p-12 text-center">
			<p class="text-lg text-gray-600">No sessions found</p>
			<p class="mt-2 text-sm text-gray-500">
				{filter === 'upcoming' ? 'No upcoming sessions available' : 'No sessions to display'}
			</p>
		</div>
	{:else}
		<div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
			{#each sessions as session}
				{@const status = getSessionStatus(session)}
				<Card class="overflow-hidden transition-shadow hover:shadow-lg">
					<div class="p-6">
						<div class="mb-3 flex items-start justify-between">
							<h3 class="text-xl font-semibold text-gray-900">{session.title}</h3>
							<span class={`rounded-full px-2 py-1 text-xs font-medium ${getStatusColor(status)}`}>
								{status}
							</span>
						</div>

						{#if session.description}
							<p class="mb-4 text-sm text-gray-600">{session.description}</p>
						{/if}

						<div class="space-y-2 text-sm">
							<div class="flex items-center text-gray-600">
								<svg class="mr-2 h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"
									/>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"
									/>
								</svg>
								{session.location}
							</div>

							<div class="flex items-center text-gray-600">
								<svg class="mr-2 h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
									/>
								</svg>
								{formatDate(session.start_time)}
							</div>

							<div class="flex items-center text-gray-600">
								<svg class="mr-2 h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"
									/>
								</svg>
								{session.available_slots} / {session.max_slots} spots available
							</div>

							<div class="flex items-center font-semibold text-primary">
								<svg class="mr-2 h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
									/>
								</svg>
								{formatCurrency(session.price_vnd)}
							</div>
						</div>
					</div>

					<div class="border-t bg-gray-50 p-4">
						<Button
							class="w-full"
							variant={status === 'full' ? 'outline' : 'default'}
							disabled={status === 'full' || status === 'completed'}
							onclick={() => goto(`/sessions/${session.id}`)}
						>
							{status === 'full' ? 'Fully Booked' : status === 'completed' ? 'Completed' : 'View Details'}
						</Button>
					</div>
				</Card>
			{/each}
		</div>
	{/if}
</div>
