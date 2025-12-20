<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { api } from '$lib/api/client';
	import { formatCurrency, formatDate } from '$lib/utils';
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

	function getSessionStatus(session: Session): { label: string; variant: 'session' } {
		if (session.cancelled) {
			return { label: 'Cancelled', variant: 'session' };
		}

		if (session.available_slots === 0) {
			return { label: 'Full', variant: 'session' };
		}

		const now = new Date();
		const sessionDate = new Date(`${session.date}T${session.time}`);

		if (now < sessionDate) {
			return { label: 'Upcoming', variant: 'session' };
		}

		return { label: 'Completed', variant: 'session' };
	}

	$effect(() => {
		if (filter) {
			loadSessions();
		}
	});
</script>

<svelte:head>
	<title>Sessions - Loafy Club</title>
	<link rel="preconnect" href="https://fonts.googleapis.com">
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous">
	<link href="https://fonts.googleapis.com/css2?family=Baloo+2:wght@400;500;600;700;800&display=swap" rel="stylesheet">
</svelte:head>

<PageBackground variant="subtle">
	<Navigation />

	<div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
		<AnimatedContainer animation="fade-up">
			<SectionHeader
				title="Pickleball Sessions"
				subtitle="Find and book available sessions at Loafy Club"
				gradient
				centered
			/>
		</AnimatedContainer>

		<!-- Filters -->
		<AnimatedContainer animation="fade-up" delay={100}>
			<div class="mb-8 flex justify-center gap-2">
				<Button
					variant={filter === 'upcoming' ? 'default' : 'outline'}
					onclick={() => (filter = 'upcoming')}
					class={filter === 'upcoming' ? 'bg-gradient-to-r from-orange-500 to-pink-500 border-0' : ''}
				>
					Upcoming Sessions
				</Button>
				<Button
					variant={filter === 'all' ? 'default' : 'outline'}
					onclick={() => (filter = 'all')}
					class={filter === 'all' ? 'bg-gradient-to-r from-orange-500 to-pink-500 border-0' : ''}
				>
					All Sessions
				</Button>
			</div>
		</AnimatedContainer>

		{#if loading}
			<LoadingSpinner text="Loading sessions..." />
		{:else if error}
			<ErrorState message={error} onRetry={loadSessions} />
		{:else if sessions.length === 0}
			<EmptyState
				title="No Sessions Found"
				description={filter === 'upcoming'
					? 'There are no upcoming sessions available at the moment.'
					: 'No sessions to display.'}
				actionText="Check back later"
			/>
		{:else}
			<div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
				{#each sessions as session, i}
					{@const status = getSessionStatus(session)}
					<AnimatedContainer animation="fade-up" delay={150 + i * 50} trigger="scroll">
						<GlassCard hover variant={i % 3 === 0 ? 'yellow' : i % 3 === 1 ? 'orange' : 'pink'}>
							<div class="flex items-start justify-between gap-2 mb-4">
								<h3 class="text-lg font-bold text-gray-800 line-clamp-1" style="font-family: 'Baloo 2', sans-serif;">
									{session.title}
								</h3>
								<StatusBadge status={status.label} variant="session" />
							</div>

							<div class="flex items-center gap-1 text-sm text-gray-600 mb-4">
								<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
								</svg>
								{session.location}
							</div>

							<div class="space-y-3">
								<!-- Date & Time -->
								<div class="flex items-center gap-2 text-sm">
									<svg class="h-4 w-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
									</svg>
									<span class="text-gray-800">{formatDate(`${session.date}T${session.time}`)}</span>
									<span class="text-gray-500">at {session.time.slice(0, 5)}</span>
								</div>

								<!-- Courts & Players -->
								<div class="flex items-center gap-2 text-sm">
									<svg class="h-4 w-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
									</svg>
									<span class="text-gray-800">{session.courts} court{session.courts > 1 ? 's' : ''}</span>
									<span class="text-gray-500">&middot; {session.max_players_per_court} per court</span>
								</div>

								<!-- Availability -->
								<div class="flex items-center gap-2 text-sm">
									<svg class="h-4 w-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
									</svg>
									<span class={session.available_slots > 0 ? 'text-green-600 font-medium' : 'text-red-600 font-medium'}>
										{session.available_slots} / {session.total_slots}
									</span>
									<span class="text-gray-500">spots available</span>
								</div>

								<!-- Price -->
								<div class="pt-3 border-t border-gray-200">
									<div class="flex items-center justify-between">
										<span class="text-sm text-gray-500">Price per player</span>
										<span class="text-lg font-bold text-transparent bg-clip-text bg-gradient-to-r from-orange-500 to-pink-500">
											{formatCurrency(session.price_vnd)}
										</span>
									</div>
								</div>
							</div>

							<Button
								class="w-full mt-4 {status.label === 'Full' || status.label === 'Completed' ? '' : 'bg-gradient-to-r from-orange-500 to-pink-500 border-0'}"
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
						</GlassCard>
					</AnimatedContainer>
				{/each}
			</div>
		{/if}
	</div>
</PageBackground>
