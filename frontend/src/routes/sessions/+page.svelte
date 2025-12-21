<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { api } from '$lib/api/client';
	import { formatCurrency, formatDate, getSessionDateTime, extractErrorMessage } from '$lib/utils';
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
	import { MapPin, Calendar, LayoutGrid, Users } from 'lucide-svelte';
	import type { Session } from '$lib/types';

	const t = useTranslation();

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
		} catch (err: unknown) {
			error = extractErrorMessage(err, 'Failed to load sessions');
		} finally {
			loading = false;
		}
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

	<div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
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
			<div class="mb-8 flex justify-center gap-2">
				<Button
					variant={filter === 'upcoming' ? 'gradient' : 'outline'}
					onclick={() => (filter = 'upcoming')}
				>
					{t('sessions.upcomingSessions')}
				</Button>
				<Button
					variant={filter === 'all' ? 'gradient' : 'outline'}
					onclick={() => (filter = 'all')}
				>
					{t('sessions.allSessions')}
				</Button>
			</div>
		</AnimatedContainer>

		{#if loading}
			<LoadingSpinner text={t('sessions.loadingText')} />
		{:else if error}
			<ErrorState message={error} onRetry={loadSessions} />
		{:else if sessions.length === 0}
			<EmptyState
				title={t('sessions.noSessions')}
				description={filter === 'upcoming'
					? t('sessions.noUpcoming')
					: t('sessions.noSessionsDesc')}
				actionText={t('sessions.checkBack')}
			/>
		{:else}
			<div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
				{#each sessions as session, i}
					{@const status = getSessionStatus(session)}
					<AnimatedContainer animation="fade-up" delay={150 + i * 50} trigger="scroll">
						<GlassCard hover variant={i % 3 === 0 ? 'yellow' : i % 3 === 1 ? 'orange' : 'pink'}>
							<div class="flex items-start justify-between gap-2 mb-4">
								<h3 class="text-lg font-bold text-gray-800 line-clamp-1 font-display">
									{session.title}
								</h3>
								<StatusBadge status={status.label} statusKey={status.key} variant="session" />
							</div>

							<div class="flex items-center gap-1 text-sm text-gray-600 mb-4">
								<MapPin class="h-4 w-4" />
								{session.location}
							</div>

							<div class="space-y-3">
								<!-- Date & Time -->
								<div class="flex items-center gap-2 text-sm">
									<Calendar class="h-4 w-4 text-gray-400" />
									<span class="text-gray-800">{formatDate(`${session.date}T${session.time}`)}</span>
									<span class="text-gray-500">{t('sessions.at')} {session.time.slice(0, 5)}</span>
								</div>

								<!-- Courts & Players -->
								<div class="flex items-center gap-2 text-sm">
									<LayoutGrid class="h-4 w-4 text-gray-400" />
									<span class="text-gray-800">{session.courts} {session.courts > 1 ? t('sessions.courts') : t('sessions.court')}</span>
									<span class="text-gray-500">&middot; {session.max_players_per_court} {t('sessions.perCourt')}</span>
								</div>

								<!-- Availability -->
								<div class="flex items-center gap-2 text-sm">
									<Users class="h-4 w-4 text-gray-400" />
									<span class={session.available_slots > 0 ? 'text-green-600 font-medium' : 'text-red-600 font-medium'}>
										{session.available_slots} / {session.total_slots}
									</span>
									<span class="text-gray-500">{t('sessions.spotsAvailable')}</span>
								</div>

								<!-- Price -->
								<div class="pt-3 border-t border-gray-200">
									<div class="flex items-center justify-between">
										<span class="text-sm text-gray-500">{t('sessions.pricePerPlayer')}</span>
										<span class="text-lg font-bold text-gradient-primary">
											{formatCurrency(session.price_vnd)}
										</span>
									</div>
								</div>
							</div>

							<Button
								class="w-full mt-4"
								variant={status.key === 'full' || status.key === 'completed' ? 'outline' : 'gradient'}
								disabled={status.key === 'full' || status.key === 'completed' || status.key === 'cancelled'}
								onclick={() => goto(`/sessions/${session.id}`)}
							>
								{#if status.key === 'full'}
									{t('sessions.fullyBooked')}
								{:else if status.key === 'completed'}
									{t('sessions.sessionEnded')}
								{:else if status.key === 'cancelled'}
									{t('sessions.cancelled')}
								{:else}
									{t('sessions.book')}
								{/if}
							</Button>
						</GlassCard>
					</AnimatedContainer>
				{/each}
			</div>
		{/if}
	</div>
</PageBackground>
