<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { api } from '$lib/api/client';
	import { formatCurrency, formatDate } from '$lib/utils';
	import Navigation from '$lib/components/Navigation.svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import { Badge } from '$lib/components/ui/badge';
	import type { Session } from '$lib/types';

	let sessions = $state<Session[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);
	let filter = $state<'upcoming' | 'all'>('upcoming');

	onMount(async () => {
		await loadSessions();
	});

	async function loadSessions() {
		loading = true;
		error = null;

		try {
			const params: any = {};

			if (filter === 'upcoming') {
				const today = new Date().toISOString().split('T')[0];
				params.from_date = today;
				params.available_only = true;
			}

			const response = await api.sessions.list(params);
			sessions = response.data;
		} catch (err: any) {
			error = err.response?.data?.message || err.message || 'Failed to load sessions';
		} finally {
			loading = false;
		}
	}

	function getSessionStatus(session: Session): { label: string; variant: 'default' | 'secondary' | 'destructive' | 'outline' | 'success' | 'warning' } {
		if (session.cancelled) {
			return { label: 'Cancelled', variant: 'destructive' };
		}

		if (session.available_slots === 0) {
			return { label: 'Full', variant: 'warning' };
		}

		const now = new Date();
		const sessionDate = new Date(`${session.date}T${session.time}`);

		if (now < sessionDate) {
			return { label: 'Upcoming', variant: 'success' };
		}

		return { label: 'Completed', variant: 'secondary' };
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

<div class="min-h-screen">
	<!-- Header with gradient -->
	<div class="bg-loafy-gradient-hero py-12">
		<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 text-center">
			<h1 class="text-4xl font-bold tracking-tight text-foreground sm:text-5xl">
				<span class="text-gradient-loafy">Pickleball Sessions</span>
			</h1>
			<p class="mt-4 text-lg text-muted-foreground">
				Find and book available sessions at Loafy Club
			</p>
		</div>
	</div>

	<div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">

		<!-- Filters -->
		<div class="mb-8 flex justify-center gap-2">
			<Button
				variant={filter === 'upcoming' ? 'default' : 'outline'}
				onclick={() => (filter = 'upcoming')}
			>
				Upcoming Sessions
			</Button>
			<Button
				variant={filter === 'all' ? 'default' : 'outline'}
				onclick={() => (filter = 'all')}
			>
				All Sessions
			</Button>
		</div>

		<!-- Loading State -->
		{#if loading}
			<div class="flex flex-col items-center justify-center py-16">
				<div class="h-12 w-12 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
				<p class="mt-4 text-muted-foreground">Loading sessions...</p>
			</div>

		<!-- Error State -->
		{:else if error}
			<Card.Root class="mx-auto max-w-md">
				<Card.Header>
					<Card.Title class="text-destructive">Error</Card.Title>
					<Card.Description>{error}</Card.Description>
				</Card.Header>
				<Card.Footer>
					<Button onclick={loadSessions} class="w-full">Try Again</Button>
				</Card.Footer>
			</Card.Root>

		<!-- Empty State -->
		{:else if sessions.length === 0}
			<Card.Root class="mx-auto max-w-md text-center">
				<Card.Header>
					<Card.Title>No Sessions Found</Card.Title>
					<Card.Description>
						{filter === 'upcoming'
							? 'There are no upcoming sessions available at the moment.'
							: 'No sessions to display.'}
					</Card.Description>
				</Card.Header>
				<Card.Content>
					<p class="text-sm text-muted-foreground">
						Check back later for new sessions!
					</p>
				</Card.Content>
			</Card.Root>

		<!-- Sessions Grid -->
		{:else}
			<div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
				{#each sessions as session}
					{@const status = getSessionStatus(session)}
					<Card.Root class="group overflow-hidden transition-all hover:shadow-lg hover:shadow-primary/10 hover:-translate-y-1 border-border/50 hover:border-primary/30">
						<Card.Header class="pb-4">
							<div class="flex items-start justify-between gap-2">
								<Card.Title class="text-lg line-clamp-1">{session.title}</Card.Title>
								<Badge variant={status.variant}>{status.label}</Badge>
							</div>
							<Card.Description class="flex items-center gap-1">
								<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
								</svg>
								{session.location}
							</Card.Description>
						</Card.Header>

						<Card.Content class="space-y-3">
							<!-- Date & Time -->
							<div class="flex items-center gap-2 text-sm">
								<svg class="h-4 w-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
								</svg>
								<span class="text-foreground">{formatDate(`${session.date}T${session.time}`)}</span>
								<span class="text-muted-foreground">at {session.time.slice(0, 5)}</span>
							</div>

							<!-- Courts & Players -->
							<div class="flex items-center gap-2 text-sm">
								<svg class="h-4 w-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
								</svg>
								<span class="text-foreground">{session.courts} court{session.courts > 1 ? 's' : ''}</span>
								<span class="text-muted-foreground">&middot; {session.max_players_per_court} per court</span>
							</div>

							<!-- Availability -->
							<div class="flex items-center gap-2 text-sm">
								<svg class="h-4 w-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
								</svg>
								<span class={session.available_slots > 0 ? 'text-green-600 font-medium' : 'text-destructive font-medium'}>
									{session.available_slots} / {session.total_slots}
								</span>
								<span class="text-muted-foreground">spots available</span>
							</div>

							<!-- Price -->
							<div class="pt-2 border-t">
								<div class="flex items-center justify-between">
									<span class="text-sm text-muted-foreground">Price per player</span>
									<span class="text-lg font-bold text-primary">{formatCurrency(session.price_vnd)}</span>
								</div>
							</div>
						</Card.Content>

						<Card.Footer class="pt-0">
							<Button
								class="w-full"
								variant={status.label === 'Full' || status.label === 'Completed' ? 'outline' : 'default'}
								disabled={status.label === 'Full' || status.label === 'Completed' || status.label === 'Cancelled'}
								onclick={() => goto(`/sessions/${session.id}`)}
							>
								{#if status.label === 'Full'}
									Fully Booked
								{:else if status.label === 'Completed'}
									Session Ended
								{:else if status.label === 'Cancelled'}
									Cancelled
								{:else}
									Book Now
								{/if}
							</Button>
						</Card.Footer>
					</Card.Root>
				{/each}
			</div>
		{/if}
	</div>
</div>
