<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { api } from '$lib/api/client';
	import { formatCurrency, formatDate, getSessionDateTime, extractErrorMessage } from '$lib/utils';
	import { useTranslation } from '$lib/i18n/index.svelte';
	import Navigation from '$lib/components/Navigation.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Card } from '$lib/components/ui/card';
	import { PageBackground } from '$lib/components/ui/page-background';
	import { SectionHeader } from '$lib/components/ui/section-header';
	import { SessionCardSkeleton } from '$lib/components/ui/skeleton';
	import * as Empty from '$lib/components/ui/empty';
	import { Alert, AlertDescription } from '$lib/components/ui/alert';
	import { Badge } from '$lib/components/ui/badge';
	import { AnimatedContainer } from '$lib/components/ui/animated-container';
	import * as Tabs from '$lib/components/ui/tabs';
	import { SessionParticipants } from '$lib/components/ui/session-participants';
	import { MapPin, Calendar, LayoutGrid } from 'lucide-svelte';
	import type { Session } from '$lib/types';

	const t = useTranslation();

	let sessions = $state<Session[]>([]);
	let bookedSessionIds = $state<Set<string>>(new Set());
	let loading = $state(true);
	let showSkeleton = $state(false);
	let error = $state<string | null>(null);
	let filter = $state<'upcoming' | 'all' | undefined>('upcoming');

	onMount(async () => {
		// Only show skeleton if loading takes longer than 200ms
		const skeletonTimer = setTimeout(() => {
			if (loading) showSkeleton = true;
		}, 200);

		await Promise.all([loadSessions(), loadUserBookings()]);
		clearTimeout(skeletonTimer);
	});

	async function loadUserBookings() {
		try {
			const response = await api.bookings.list({ per_page: 100 });
			const bookings = response.data.data || response.data;
			// Extract session IDs from active (non-cancelled) bookings
			const sessionIds = new Set<string>();
			for (const booking of bookings) {
				if (!booking.cancelled_at) {
					sessionIds.add(booking.session_id);
				}
			}
			bookedSessionIds = sessionIds;
		} catch {
			// Silently fail - user might not be logged in
		}
	}

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
		} catch (err: unknown) {
			error = extractErrorMessage(err, 'Failed to load sessions');
		} finally {
			loading = false;
		}
	}

	function isSessionBooked(sessionId: string): boolean {
		return bookedSessionIds.has(sessionId);
	}

	function getSessionStatus(session: Session): { label: string; key: string; variant: 'session' } {
		if (session.cancelled) {
			return { label: t('sessions.status.cancelled'), key: 'cancelled', variant: 'session' };
		}

		if (session.available_slots === 0) {
			return { label: t('sessions.status.full'), key: 'full', variant: 'session' };
		}

		const now = new Date();
		const sessionDate = getSessionDateTime(session);

		if (now < sessionDate) {
			return { label: t('sessions.status.upcoming'), key: 'upcoming', variant: 'session' };
		}

		return { label: t('sessions.status.completed'), key: 'completed', variant: 'session' };
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

<PageBackground variant="subtle">
	<Navigation />

	<main class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
		<AnimatedContainer animation="fade-up">
			<SectionHeader
				title={t('sessions.pageTitle')}
				subtitle={t('sessions.pageSubtitle')}
				gradient
				centered
			/>
		</AnimatedContainer>

		<!-- Filters -->
		<AnimatedContainer animation="fade-up" delay={100}>
			<div class="mb-8 flex justify-center">
				<Tabs.Root bind:value={filter}>
					<Tabs.List class="h-10">
						<Tabs.Trigger value="upcoming" class="px-6 data-[state=active]:bg-gradient-to-r data-[state=active]:from-orange-500 data-[state=active]:to-pink-500 data-[state=active]:text-white data-[state=active]:border-transparent">
							{t('sessions.upcomingSessions')}
						</Tabs.Trigger>
						<Tabs.Trigger value="all" class="px-6 data-[state=active]:bg-gradient-to-r data-[state=active]:from-orange-500 data-[state=active]:to-pink-500 data-[state=active]:text-white data-[state=active]:border-transparent">
							{t('sessions.allSessions')}
						</Tabs.Trigger>
					</Tabs.List>
				</Tabs.Root>
			</div>
		</AnimatedContainer>

		{#if loading && showSkeleton}
			<div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
				{#each Array(8) as _, i}
					<SessionCardSkeleton variant={i % 3 === 0 ? 'glassYellow' : i % 3 === 1 ? 'glassOrange' : 'glassPink'} />
				{/each}
			</div>
		{:else if loading}
			<!-- Brief loading, no skeleton needed -->
		{:else if error}
			<AnimatedContainer animation="fade-up" delay={100}>
				<Card variant="glass" class="mx-auto max-w-md">
					<Alert variant="destructive">
						<AlertDescription>{error}</AlertDescription>
					</Alert>
					<Button class="mt-4 w-full" onclick={loadSessions}>{t('common.retry')}</Button>
				</Card>
			</AnimatedContainer>
		{:else if sessions.length === 0}
			<AnimatedContainer animation="fade-up" delay={100}>
				<Card variant="glass" class="mx-auto max-w-md">
					<Empty.Root>
						<Empty.Header>
							<Empty.Media variant="icon">
								<Calendar class="size-5" />
							</Empty.Media>
							<Empty.Title>{t('sessions.noSessions')}</Empty.Title>
							<Empty.Description>
								{filter === 'upcoming' ? t('sessions.noUpcoming') : t('sessions.noSessionsDesc')}
							</Empty.Description>
						</Empty.Header>
					</Empty.Root>
				</Card>
			</AnimatedContainer>
		{:else}
			<div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
				{#each sessions as session, i}
					{@const status = getSessionStatus(session)}
					<AnimatedContainer animation="fade-up" delay={150 + i * 50} trigger="scroll">
						<Card hover variant={i % 3 === 0 ? 'glassYellow' : i % 3 === 1 ? 'glassOrange' : 'glassPink'}>
							<div class="flex items-start justify-between gap-2 mb-4">
								<h3 class="text-lg font-bold text-foreground line-clamp-1 font-display">
									{session.title}
								</h3>
								<Badge variant={status.key as any}>{status.label}</Badge>
							</div>

							<div class="flex items-center gap-1 text-sm text-muted-foreground mb-4">
								<MapPin class="h-4 w-4" />
								{session.location}
							</div>

							<div class="space-y-3">
								<!-- Date & Time -->
								<div class="flex items-center gap-2 text-sm">
									<Calendar class="h-4 w-4 text-muted-foreground" />
									<span class="text-foreground">{formatDate(`${session.date}T${session.time}`)}</span>
									<span class="text-muted-foreground">{t('sessions.at')} {session.time.slice(0, 5)}</span>
								</div>

								<!-- Courts & Players -->
								<div class="flex items-center gap-2 text-sm">
									<LayoutGrid class="h-4 w-4 text-muted-foreground" />
									<span class="text-foreground">{session.courts} {session.courts > 1 ? t('sessions.courts') : t('sessions.court')}</span>
									<span class="text-muted-foreground">&middot; {session.max_players_per_court} {t('sessions.perCourt')}</span>
								</div>

								<!-- Participants & Availability -->
								<SessionParticipants session={session} variant="default" showIcon class="text-sm" />

								<!-- Price -->
								<div class="pt-3 border-t border-border">
									<div class="flex items-center justify-between">
										<span class="text-sm text-muted-foreground">{t('sessions.pricePerPlayer')}</span>
										<span class="text-lg font-bold text-gradient-primary">
											{formatCurrency(session.price_vnd)}
										</span>
									</div>
								</div>
							</div>

							{@const alreadyBooked = isSessionBooked(session.id)}
							<Button
								class="w-full mt-4"
								variant={status.key === 'full' || status.key === 'completed' || alreadyBooked ? 'outline' : 'gradient'}
								disabled={status.key === 'full' || status.key === 'completed' || status.key === 'cancelled' || alreadyBooked}
								onclick={() => goto(`/sessions/${session.id}`)}
							>
								{#if alreadyBooked}
									{t('sessions.alreadyBooked')}
								{:else if status.key === 'full'}
									{t('sessions.fullyBooked')}
								{:else if status.key === 'completed'}
									{t('sessions.sessionEnded')}
								{:else if status.key === 'cancelled'}
									{t('sessions.cancelled')}
								{:else}
									{t('sessions.book')}
								{/if}
							</Button>
						</Card>
					</AnimatedContainer>
				{/each}
			</div>
		{/if}
	</main>
</PageBackground>
