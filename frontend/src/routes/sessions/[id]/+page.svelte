<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { api } from '$lib/api/client';
	import {
		formatCurrency,
		formatDate,
		canBookSession,
		getSessionDateTime,
		extractErrorMessage
	} from '$lib/utils';
	import { requireAuth } from '$lib/guards/auth';
	import { useTranslation } from '$lib/i18n/index.svelte';
	import Navigation from '$lib/components/Navigation.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Card } from '$lib/components/ui/card';
	import { PageBackground } from '$lib/components/ui/page-background';
	import { SessionDetailSkeleton } from '$lib/components/ui/skeleton';
	import * as Empty from '$lib/components/ui/empty';
	import { AnimatedContainer } from '$lib/components/ui/animated-container';
	import { Label } from '$lib/components/ui/label';
	import * as Select from '$lib/components/ui/select';
	import { RadioGroup, RadioGroupItem } from '$lib/components/ui/radio-group';
	import { Alert, AlertDescription } from '$lib/components/ui/alert';
	import { MapPin, Calendar, Users, DollarSign, ArrowLeft, AlertCircle } from 'lucide-svelte';
	import { SessionParticipants } from '$lib/components/ui/session-participants';
	import type { Session } from '$lib/types';

	const t = useTranslation();

	let sessionId = $derived($page.params.id);
	let session = $state<Session | null>(null);
	let hasExistingBooking = $state(false);
	let loading = $state(true);
	let showSkeleton = $state(false);
	let error = $state<string | null>(null);
	let bookingLoading = $state(false);
	let guestCount = $state(0);
	let paymentMethod = $state<'stripe' | 'qr'>('stripe');

	onMount(async () => {
		if (!(await requireAuth())) return;

		const skeletonTimer = setTimeout(() => {
			if (loading) showSkeleton = true;
		}, 200);

		await Promise.all([loadSession(), checkExistingBooking()]);
		clearTimeout(skeletonTimer);
	});

	async function checkExistingBooking() {
		if (!sessionId) return;
		try {
			const response = await api.bookings.list({ per_page: 100 });
			const bookings = response.data.data || response.data;
			hasExistingBooking = bookings.some(
				(b: any) => b.session_id === sessionId && !b.cancelled_at
			);
		} catch {
			// Silently fail
		}
	}

	async function loadSession() {
		loading = true;
		error = null;

		try {
			if (!sessionId) {
				throw new Error('Session ID is required');
			}
			const response = await api.sessions.get(sessionId);
			session = response.data;
		} catch (err: unknown) {
			error = extractErrorMessage(err, 'Failed to load session');
		} finally {
			loading = false;
		}
	}

	async function handleBooking() {
		if (!session || !sessionId) return;

		const totalSlots = 1 + guestCount;
		if (totalSlots > session.available_slots) {
			error = `Only ${session.available_slots} spots available`;
			return;
		}

		bookingLoading = true;
		error = null;

		try {
			const response = await api.bookings.create({
				session_id: sessionId,
				guest_count: guestCount,
				payment_method: paymentMethod
			});

			goto(`/bookings/${response.data.id}/payment`);
		} catch (err: unknown) {
			error = extractErrorMessage(err, 'Failed to create booking');
			bookingLoading = false;
		}
	}

	function getTotalPrice(): number {
		if (!session) return 0;
		return session.price_vnd * (1 + guestCount);
	}

	function getSessionStatusText(): string {
		if (!session) return '';

		const now = new Date();
		const startTime = getSessionDateTime(session);

		if (session.cancelled) {
			return t('sessionDetail.sessionCancelled');
		}

		if (session.available_slots === 0) {
			return t('sessionDetail.sessionFull');
		}

		if (now >= startTime) {
			return t('sessionDetail.sessionStarted');
		}

		return '';
	}
</script>

<svelte:head>
	<title>{session?.title || 'Session'} - Loafy Club</title>
</svelte:head>

<PageBackground variant="subtle">
	<Navigation />

	<main class="mx-auto max-w-4xl px-4 py-8 sm:px-6 lg:px-8">
		{#if loading && showSkeleton}
			<SessionDetailSkeleton />
		{:else if loading}
			<!-- Brief loading -->
		{:else if error && !session}
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
							<Button onclick={() => goto('/sessions')}>{t('sessionDetail.backToSessions')}</Button>
						</Empty.Content>
					</Empty.Root>
				</Card>
			</AnimatedContainer>
		{:else if session}
			<AnimatedContainer animation="fade-up">
				<div class="mb-6">
					<Button variant="ghost" onclick={() => goto('/sessions')}>
						<ArrowLeft class="size-4 mr-2" />
						{t('sessionDetail.backToSessions')}
					</Button>
				</div>
			</AnimatedContainer>

			<div class="grid gap-6 lg:grid-cols-3">
				<div class="lg:col-span-2">
					<AnimatedContainer animation="fade-up" delay={100}>
						<Card variant="glass">
							<h1 class="text-3xl font-bold font-display text-foreground">
								{session.title}
							</h1>

							{#if session.description}
								<p class="mt-4 text-muted-foreground">{session.description}</p>
							{/if}

							<div class="mt-6 space-y-4">
								<div class="flex items-start">
									<MapPin class="mr-3 h-6 w-6 text-orange-400" />
									<div>
										<p class="font-medium text-foreground">{t('sessionDetail.location')}</p>
										<p class="text-muted-foreground">{session.location}</p>
									</div>
								</div>

								<div class="flex items-start">
									<Calendar class="mr-3 h-6 w-6 text-orange-400" />
									<div>
										<p class="font-medium text-foreground">{t('sessionDetail.dateTime')}</p>
										<p class="text-muted-foreground">{session.date} {t('sessions.at')} {session.time}</p>
									</div>
								</div>

								<div class="flex items-start">
									<Users class="mr-3 h-6 w-6 text-orange-400" />
									<div>
										<p class="font-medium text-foreground">{t('sessionDetail.participants')}</p>
										<div class="mt-1">
											<SessionParticipants session={session} variant="detailed" />
										</div>
									</div>
								</div>

								<div class="flex items-start">
									<DollarSign class="mr-3 h-6 w-6 text-orange-400" />
									<div>
										<p class="font-medium text-foreground">{t('sessionDetail.price')}</p>
									<p class="text-2xl font-bold text-gradient-primary">
										{formatCurrency(session.price_vnd)}
									</p>
										<p class="text-sm text-muted-foreground">{t('sessionDetail.perPerson')}</p>
									</div>
								</div>
							</div>
						</Card>
					</AnimatedContainer>
				</div>

				<div class="lg:col-span-1">
					<AnimatedContainer animation="fade-up" delay={200}>
						<Card variant="glass" class="sticky top-20">
						<h2 class="text-xl font-bold font-display text-foreground">
							{t('sessionDetail.bookYourSpot')}
						</h2>

							{#if hasExistingBooking}
								<Alert class="mt-4" variant="default">
									<AlertDescription>
										{t('sessionDetail.alreadyBooked')}
									</AlertDescription>
								</Alert>
								<Button
									variant="outline"
									class="w-full mt-4"
									onclick={() => goto('/bookings')}
								>
									{t('sessionDetail.viewMyBookings')}
								</Button>
							{:else if !canBookSession(session)}
								<Alert class="mt-4" variant="warning">
									<AlertDescription>
										{getSessionStatusText()}
									</AlertDescription>
								</Alert>
							{:else}
								<div class="mt-6 space-y-4">
									<div>
										<Label>{t('sessionDetail.numberOfGuests')}</Label>
										<Select.Root type="single" value={String(guestCount)} onValueChange={(value) => value && (guestCount = Number(value))}>
											<Select.Trigger class="mt-1 w-full">
												<Select.Value placeholder={t('sessionDetail.selectGuests')}>
													{guestCount === 0 ? t('sessionDetail.justMe') : t(guestCount === 1 ? 'sessionDetail.guest' : 'sessionDetail.guests', { count: guestCount })}
												</Select.Value>
											</Select.Trigger>
											<Select.Content>
												<Select.Item value="0">{t('sessionDetail.justMe')}</Select.Item>
												{#each Array(Math.min(3, session.available_slots - 1)) as _, i}
													<Select.Item value={String(i + 1)}>{t(i === 0 ? 'sessionDetail.guest' : 'sessionDetail.guests', { count: i + 1 })}</Select.Item>
												{/each}
											</Select.Content>
										</Select.Root>
										<p class="mt-1 text-xs text-muted-foreground">{t('sessionDetail.maxGuests')}</p>
									</div>

									<fieldset>
										<Label>{t('sessionDetail.paymentMethod')}</Label>
										<RadioGroup bind:value={paymentMethod} class="mt-2 gap-2">
											<label
												class="flex items-center gap-3 p-3 rounded-xl border-2 cursor-pointer hover:border-orange-300 transition-colors {paymentMethod === 'stripe' ? 'border-primary bg-primary/10' : 'border-border'}"
											>
												<RadioGroupItem value="stripe" class="border-orange-400 text-orange-500 data-[state=checked]:border-orange-500" />
												<span class="text-sm text-foreground">{t('sessionDetail.stripe')}</span>
											</label>
											<label class="flex items-center gap-3 p-3 rounded-xl border-2 border-border cursor-not-allowed opacity-50">
												<RadioGroupItem value="qr" disabled class="border-muted" />
												<span class="text-sm text-muted-foreground">{t('sessionDetail.qrComingSoon')}</span>
											</label>
										</RadioGroup>
									</fieldset>

									<div class="border-t border-border pt-4">
										<div class="flex justify-between text-sm text-muted-foreground">
											<span>{t('sessionDetail.pricePerPerson')}</span>
											<span>{formatCurrency(session.price_vnd)}</span>
										</div>
										<div class="mt-2 flex justify-between text-sm text-muted-foreground">
											<span>{t('sessionDetail.numberOfPeople')}</span>
											<span>{1 + guestCount}</span>
										</div>
										<div class="mt-2 flex justify-between text-lg font-bold text-foreground">
											<span>{t('sessionDetail.total')}</span>
										<span class="text-gradient-primary">
											{formatCurrency(getTotalPrice())}
										</span>
										</div>
									</div>

									{#if error}
										<Alert variant="destructive">
											<AlertDescription>{error}</AlertDescription>
										</Alert>
									{/if}

									<Button
										variant="gradient"
										class="w-full py-6 text-lg"
										size="lg"
										disabled={bookingLoading}
										onclick={handleBooking}
									>
										{bookingLoading ? t('sessionDetail.creatingBooking') : t('sessions.book')}
									</Button>

									<p class="text-xs text-center text-muted-foreground">
										{t('sessionDetail.paymentTime')}
									</p>
								</div>
							{/if}
						</Card>
					</AnimatedContainer>
				</div>
			</div>
		{/if}
	</main>
</PageBackground>
