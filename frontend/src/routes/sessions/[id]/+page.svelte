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
	import { GlassCard } from '$lib/components/ui/glass-card';
	import { PageBackground } from '$lib/components/ui/page-background';
	import { BackButton } from '$lib/components/ui/back-button';
	import { LoadingSpinner } from '$lib/components/ui/loading-spinner';
	import { ErrorState } from '$lib/components/ui/error-state';
	import { AnimatedContainer } from '$lib/components/ui/animated-container';
	import { Label } from '$lib/components/ui/label';
	import { SelectNative } from '$lib/components/ui/select-native';
	import { Alert, AlertDescription } from '$lib/components/ui/alert';
	import { MapPin, Calendar, Users, DollarSign } from 'lucide-svelte';
	import type { Session } from '$lib/types';

	const t = useTranslation();

	let sessionId = $derived($page.params.id);
	let session = $state<Session | null>(null);
	let loading = $state(true);
	let error = $state<string | null>(null);
	let bookingLoading = $state(false);
	let guestCount = $state(0);
	let paymentMethod = $state<'stripe' | 'qr'>('stripe');

	onMount(async () => {
		if (!requireAuth()) return;
		await loadSession();
	});

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

	<div class="mx-auto max-w-4xl px-4 py-8 sm:px-6 lg:px-8">
		{#if loading}
			<LoadingSpinner text={t('sessionDetail.loadingSession')} />
		{:else if error && !session}
			<ErrorState
				message={error}
				onRetry={() => goto('/sessions')}
				retryText={t('sessionDetail.backToSessions')}
			/>
		{:else if session}
			<AnimatedContainer animation="fade-up">
				<BackButton href="/sessions" label={t('sessionDetail.backToSessions')} />
			</AnimatedContainer>

			<div class="grid gap-6 lg:grid-cols-3">
				<div class="lg:col-span-2">
					<AnimatedContainer animation="fade-up" delay={100}>
						<GlassCard>
							<h1 class="text-3xl font-bold text-gray-800 font-display">
								{session.title}
							</h1>

							{#if session.description}
								<p class="mt-4 text-gray-600">{session.description}</p>
							{/if}

							<div class="mt-6 space-y-4">
								<div class="flex items-start">
									<MapPin class="mr-3 h-6 w-6 text-orange-400" />
									<div>
										<p class="font-medium text-gray-800">{t('sessionDetail.location')}</p>
										<p class="text-gray-600">{session.location}</p>
									</div>
								</div>

								<div class="flex items-start">
									<Calendar class="mr-3 h-6 w-6 text-orange-400" />
									<div>
										<p class="font-medium text-gray-800">{t('sessionDetail.dateTime')}</p>
										<p class="text-gray-600">{session.date} {t('sessions.at')} {session.time}</p>
									</div>
								</div>

								<div class="flex items-start">
									<Users class="mr-3 h-6 w-6 text-orange-400" />
									<div>
										<p class="font-medium text-gray-800">{t('sessionDetail.availability')}</p>
										<p class="text-gray-600">
											{t('sessionDetail.spotsOf', { available: session.available_slots, total: session.total_slots })}
										</p>
									</div>
								</div>

								<div class="flex items-start">
									<DollarSign class="mr-3 h-6 w-6 text-orange-400" />
									<div>
										<p class="font-medium text-gray-800">{t('sessionDetail.price')}</p>
									<p class="text-2xl font-bold text-gradient-primary">
										{formatCurrency(session.price_vnd)}
									</p>
										<p class="text-sm text-gray-500">{t('sessionDetail.perPerson')}</p>
									</div>
								</div>
							</div>
						</GlassCard>
					</AnimatedContainer>
				</div>

				<div class="lg:col-span-1">
					<AnimatedContainer animation="fade-up" delay={200}>
						<GlassCard class="sticky top-20">
						<h2 class="text-xl font-bold text-gray-800 font-display">
							{t('sessionDetail.bookYourSpot')}
						</h2>

							{#if !canBookSession(session)}
								<Alert class="mt-4 bg-yellow-50 border-yellow-200">
									<AlertDescription class="text-yellow-800">
										{getSessionStatusText()}
									</AlertDescription>
								</Alert>
							{:else}
								<div class="mt-6 space-y-4">
									<div>
										<Label for="guest-count">{t('sessionDetail.numberOfGuests')}</Label>
										<SelectNative
											id="guest-count"
											bind:value={guestCount}
											variant="styled"
											class="mt-1"
										>
											<option value={0}>{t('sessionDetail.justMe')}</option>
											{#each Array(Math.min(3, session.available_slots - 1)) as _, i}
												<option value={i + 1}>{i + 1} {i > 0 ? t('sessionDetail.guests', { count: i + 1 }) : t('sessionDetail.guest', { count: 1 })}</option>
											{/each}
										</SelectNative>
										<p class="mt-1 text-xs text-gray-500">{t('sessionDetail.maxGuests')}</p>
									</div>

									<fieldset>
										<Label>{t('sessionDetail.paymentMethod')}</Label>
										<div class="mt-2 space-y-2">
											<label class="flex items-center gap-3 p-3 rounded-xl border-2 border-gray-200 cursor-pointer hover:border-orange-300 transition-colors {paymentMethod === 'stripe' ? 'border-orange-400 bg-orange-50' : ''}">
												<input
													type="radio"
													bind:group={paymentMethod}
													value="stripe"
													class="h-4 w-4 text-orange-500 focus:ring-orange-400"
												/>
												<span class="text-sm text-gray-700">{t('sessionDetail.stripe')}</span>
											</label>
											<label class="flex items-center gap-3 p-3 rounded-xl border-2 border-gray-200 cursor-not-allowed opacity-50">
												<input
													type="radio"
													bind:group={paymentMethod}
													value="qr"
													class="h-4 w-4 text-orange-500 focus:ring-orange-400"
													disabled
												/>
												<span class="text-sm text-gray-500">{t('sessionDetail.qrComingSoon')}</span>
											</label>
										</div>
									</fieldset>

									<div class="border-t border-gray-200 pt-4">
										<div class="flex justify-between text-sm text-gray-600">
											<span>{t('sessionDetail.pricePerPerson')}</span>
											<span>{formatCurrency(session.price_vnd)}</span>
										</div>
										<div class="mt-2 flex justify-between text-sm text-gray-600">
											<span>{t('sessionDetail.numberOfPeople')}</span>
											<span>{1 + guestCount}</span>
										</div>
										<div class="mt-2 flex justify-between text-lg font-bold text-gray-800">
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

									<p class="text-xs text-center text-gray-500">
										{t('sessionDetail.paymentTime')}
									</p>
								</div>
							{/if}
						</GlassCard>
					</AnimatedContainer>
				</div>
			</div>
		{/if}
	</div>
</PageBackground>
