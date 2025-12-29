<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { api } from '$lib/api/client';
	import { formatCurrency, formatDateOnly, formatTime, formatDuration, getSessionDateTime, extractErrorMessage } from '$lib/utils';
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
	import { SessionParticipants } from '$lib/components/ui/session-participants';
	import { Pagination } from '$lib/components/ui/pagination';
	import { SessionFilters } from '$lib/components/sessions';
	import { MapPin, Calendar, Clock } from 'lucide-svelte';
	import {
		startOfWeek,
		endOfWeek,
		today,
		getLocalTimeZone,
		parseDate,
		type DateValue
	} from '@internationalized/date';
	import type { Session, PageInfo } from '$lib/types';

	const t = useTranslation();

	const SESSIONS_PER_PAGE = 8;

	// State
	let sessions = $state<Session[]>([]);
	let locations = $state<string[]>([]);
	let bookedSessionIds = $state<Set<string>>(new Set());
	let loading = $state(true);
	let showSkeleton = $state(false);
	let error = $state<string | null>(null);
	let currentPage = $state(1);

	// Filter state - initialized from URL
	let dateRange = $state<'this' | 'next' | 'pick' | 'past'>('this');
	let pickedDate = $state<DateValue | undefined>(undefined);
	let timeOfDay = $state<('morning' | 'afternoon' | 'evening')[]>([]);
	let locationFilter = $state('');
	let showFull = $state(false);

	// Derived
	const paginatedSessions = $derived(
		sessions.slice((currentPage - 1) * SESSIONS_PER_PAGE, currentPage * SESSIONS_PER_PAGE)
	);

	const pageInfo = $derived<PageInfo>({
		page: currentPage,
		per_page: SESSIONS_PER_PAGE,
		total: sessions.length,
		total_pages: Math.max(1, Math.ceil(sessions.length / SESSIONS_PER_PAGE))
	});

	// Initialize from URL params
	function initFromUrl() {
		const params = $page.url.searchParams;

		// Date range
		const week = params.get('week');
		if (week === 'next') dateRange = 'next';
		else if (week === 'past') dateRange = 'past';
		else if (params.get('date')) {
			dateRange = 'pick';
			try {
				pickedDate = parseDate(params.get('date')!);
			} catch {
				dateRange = 'this';
			}
		} else {
			dateRange = 'this';
		}

		// Time of day
		const timeParam = params.get('time');
		if (timeParam) {
			timeOfDay = timeParam.split(',').filter((t) =>
				['morning', 'afternoon', 'evening'].includes(t)
			) as ('morning' | 'afternoon' | 'evening')[];
		}

		// Location
		locationFilter = params.get('location') || '';

		// Show full
		showFull = params.get('showFull') === 'true';

		// Page
		const pageParam = params.get('page');
		if (pageParam) {
			currentPage = parseInt(pageParam) || 1;
		}
	}

	// Sync state to URL
	function syncToUrl() {
		const params = new URLSearchParams();

		if (dateRange === 'next') params.set('week', 'next');
		else if (dateRange === 'past') params.set('week', 'past');
		else if (dateRange === 'pick' && pickedDate) {
			params.set('date', pickedDate.toString());
		}
		// 'this' is default, no param needed

		if (timeOfDay.length > 0) params.set('time', timeOfDay.join(','));
		if (locationFilter) params.set('location', locationFilter);
		if (showFull) params.set('showFull', 'true');
		if (currentPage > 1) params.set('page', currentPage.toString());

		const queryString = params.toString();
		const newUrl = queryString ? `/sessions?${queryString}` : '/sessions';

		goto(newUrl, { replaceState: true, keepFocus: true });
	}

	// Calculate date range for API
	function getDateFilters(): { from_date?: string; to_date?: string } {
		const tz = getLocalTimeZone();
		const now = today(tz);

		if (dateRange === 'this') {
			const weekStart = startOfWeek(now, 'en-US');
			const weekEnd = endOfWeek(now, 'en-US');
			return {
				from_date: now.toString(),
				to_date: weekEnd.toString()
			};
		} else if (dateRange === 'next') {
			const nextWeek = now.add({ weeks: 1 });
			const nextWeekStart = startOfWeek(nextWeek, 'en-US');
			const nextWeekEnd = endOfWeek(nextWeek, 'en-US');
			return {
				from_date: nextWeekStart.toString(),
				to_date: nextWeekEnd.toString()
			};
		} else if (dateRange === 'pick' && pickedDate) {
			return {
				from_date: pickedDate.toString(),
				to_date: pickedDate.toString()
			};
		} else if (dateRange === 'past') {
			// Past 30 days until yesterday
			const thirtyDaysAgo = now.subtract({ days: 30 });
			const yesterday = now.subtract({ days: 1 });
			return {
				from_date: thirtyDaysAgo.toString(),
				to_date: yesterday.toString()
			};
		}

		return { from_date: now.toString() };
	}

	onMount(async () => {
		initFromUrl();

		const skeletonTimer = setTimeout(() => {
			if (loading) showSkeleton = true;
		}, 200);

		await Promise.all([loadSessions(), loadLocations(), loadUserBookings()]);
		clearTimeout(skeletonTimer);
	});

	async function loadLocations() {
		try {
			const response = await api.sessions.locations();
			locations = response.data;
		} catch {
			// Silently fail - locations are optional
		}
	}

	async function loadUserBookings() {
		try {
			const response = await api.bookings.list({ per_page: 100 });
			const bookings = response.data.data || response.data;
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
			const dateFilters = getDateFilters();
			const params: Parameters<typeof api.sessions.list>[0] = {
				...dateFilters,
				available_only: !showFull && dateRange !== 'past'
			};

			if (timeOfDay.length > 0) {
				params.time_of_day = timeOfDay.join(',');
			}

			if (locationFilter) {
				params.location = locationFilter;
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

	// Filter change handlers
	function handleDateRangeChange(range: 'this' | 'next' | 'pick' | 'past') {
		dateRange = range;
		currentPage = 1;
		syncToUrl();
		loadSessions();
	}

	function handlePickedDateChange(date: DateValue | undefined) {
		pickedDate = date;
		currentPage = 1;
		syncToUrl();
		loadSessions();
	}

	function handleTimeOfDayChange(times: ('morning' | 'afternoon' | 'evening')[]) {
		timeOfDay = times;
		currentPage = 1;
		syncToUrl();
		loadSessions();
	}

	function handleLocationChange(loc: string) {
		locationFilter = loc;
		currentPage = 1;
		syncToUrl();
		loadSessions();
	}

	function handleShowFullChange(show: boolean) {
		showFull = show;
		currentPage = 1;
		syncToUrl();
		loadSessions();
	}

	function handleClearFilters() {
		dateRange = 'this';
		pickedDate = undefined;
		timeOfDay = [];
		locationFilter = '';
		showFull = false;
		currentPage = 1;
		syncToUrl();
		loadSessions();
	}

	function handlePageChange(page: number) {
		currentPage = page;
		syncToUrl();
	}
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
			<div class="mb-6">
				<SessionFilters
					{dateRange}
					{pickedDate}
					{timeOfDay}
					location={locationFilter}
					{locations}
					{showFull}
					onDateRangeChange={handleDateRangeChange}
					onPickedDateChange={handlePickedDateChange}
					onTimeOfDayChange={handleTimeOfDayChange}
					onLocationChange={handleLocationChange}
					onShowFullChange={handleShowFullChange}
					onClearFilters={handleClearFilters}
				/>
			</div>
		</AnimatedContainer>

		<!-- Results Count -->
		{#if !loading && sessions.length > 0}
			<AnimatedContainer animation="fade-up" delay={120}>
				<p class="mb-4 text-sm text-muted-foreground">
					{t('sessions.filters.results', { count: sessions.length })}
				</p>
			</AnimatedContainer>
		{/if}

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
								{t('sessions.filters.noResults')}
							</Empty.Description>
						</Empty.Header>
						<Empty.Content>
							<Button variant="outline" onclick={handleClearFilters}>
								{t('sessions.filters.clearAll')}
							</Button>
						</Empty.Content>
					</Empty.Root>
				</Card>
			</AnimatedContainer>
		{:else}
			<div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
				{#each paginatedSessions as session, i}
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
								<!-- Date -->
								<div class="flex items-center gap-2 text-sm">
									<Calendar class="h-4 w-4 text-muted-foreground" />
									<span class="text-foreground font-medium">{formatDateOnly(session.date)}</span>
								</div>

								<!-- Time & Duration -->
								<div class="flex items-center gap-2 text-sm">
									<Clock class="h-4 w-4 text-muted-foreground" />
									<span class="text-foreground">{formatTime(session.time)}</span>
									{#if session.end_time}
										{@const duration = formatDuration(session.time, session.end_time)}
										{#if duration}
											<span class="text-muted-foreground">({duration})</span>
										{/if}
									{/if}
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

			{#if pageInfo.total_pages > 1}
				<AnimatedContainer animation="fade-up" delay={200}>
					<div class="mt-8">
						<Pagination {pageInfo} onPageChange={handlePageChange} />
					</div>
				</AnimatedContainer>
			{/if}
		{/if}
	</main>
</PageBackground>
