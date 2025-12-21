<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { authStore } from '$lib/stores/auth.svelte';
	import { themeStore } from '$lib/stores/theme.svelte';
	import { i18n, useTranslation, type Locale } from '$lib/i18n/index.svelte';
	import { api } from '$lib/api/client';
	import { extractErrorMessage } from '$lib/utils';
	import Navigation from '$lib/components/Navigation.svelte';
	import { PageBackground } from '$lib/components/ui/page-background';
	import { GlassCard } from '$lib/components/ui/glass-card';
	import { SectionHeader } from '$lib/components/ui/section-header';
	import { AnimatedContainer } from '$lib/components/ui/animated-container';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { SelectNative } from '$lib/components/ui/select-native';
	import { LoadingSpinner } from '$lib/components/ui/loading-spinner';
	import { Check, X, User, Settings, Shield, Link2, Unlink2 } from 'lucide-svelte';
	import { siGoogle, siFacebook } from 'simple-icons';

	type TabId = 'profile' | 'preferences' | 'account';
	type Theme = 'light' | 'dark' | 'system';

	let activeTab = $state<TabId>('profile');
	let saving = $state(false);
	let saveSuccess = $state(false);
	let saveError = $state<string | null>(null);

	// Social linking state
	let linkingProvider = $state<string | null>(null);
	let unlinkingProvider = $state<string | null>(null);
	let linkError = $state<string | null>(null);

	// Delete account state
	let showDeleteConfirm = $state(false);
	let deleteConfirmText = $state('');
	let deleting = $state(false);
	let deleteError = $state<string | null>(null);

	// Profile form state
	let name = $state('');
	let phone = $state('');

	// Preferences state
	let emailNotifications = $state(true);

	// Theme is managed by themeStore
	let theme = $derived(themeStore.theme);

	const t = useTranslation();

	// Initialize form values from auth store
	$effect(() => {
		if (authStore.user) {
			name = authStore.user.name || '';
			phone = authStore.user.phone || '';
		}
	});

	// Load preferences from localStorage
	onMount(() => {
		// Redirect if not authenticated
		if (!authStore.loading && !authStore.isAuthenticated) {
			goto('/auth/login');
			return;
		}

		// Load email notifications preference
		const savedNotifications = localStorage.getItem('loafy_email_notifications');
		if (savedNotifications !== null) emailNotifications = savedNotifications === 'true';
	});

	function setTheme(newTheme: Theme) {
		themeStore.setTheme(newTheme);
	}

	async function saveProfile() {
		saving = true;
		saveError = null;
		saveSuccess = false;

		try {
			const response = await api.users.updateProfile({
				name: name || undefined,
				phone: phone || undefined
			});

			// Update auth store with new user data
			if (authStore.user) {
				authStore.user = { ...authStore.user, ...response.data };
			}

			saveSuccess = true;
			setTimeout(() => (saveSuccess = false), 3000);
		} catch (err: unknown) {
			saveError = extractErrorMessage(err, 'Failed to save profile');
		} finally {
			saving = false;
		}
	}

	function savePreferences() {
		localStorage.setItem('loafy_email_notifications', String(emailNotifications));

		saveSuccess = true;
		setTimeout(() => (saveSuccess = false), 3000);
	}

	async function handleSignOut() {
		await authStore.signOut();
		goto('/');
	}

	async function handleDeleteAccount() {
		if (deleteConfirmText !== 'DELETE') {
			deleteError = t('account.delete.confirmMismatch');
			return;
		}

		deleting = true;
		deleteError = null;

		try {
			await authStore.deleteAccount();
			goto('/');
		} catch (err: unknown) {
			deleteError = extractErrorMessage(err, t('account.delete.failed'));
			deleting = false;
		}
	}

	function cancelDelete() {
		showDeleteConfirm = false;
		deleteConfirmText = '';
		deleteError = null;
	}

	async function handleLinkProvider(provider: 'google' | 'facebook') {
		linkingProvider = provider;
		linkError = null;
		try {
			await authStore.linkIdentity(provider);
		} catch (err: unknown) {
			linkError = extractErrorMessage(err, `Failed to link ${provider}`);
			linkingProvider = null;
		}
	}

	async function handleUnlinkProvider(provider: 'google' | 'facebook') {
		// Prevent unlinking if it's the only identity
		if (authStore.identities.length <= 1) {
			linkError = t('account.connected.cannotUnlinkLast');
			return;
		}

		unlinkingProvider = provider;
		linkError = null;
		try {
			await authStore.unlinkIdentity(provider);
			saveSuccess = true;
			setTimeout(() => (saveSuccess = false), 3000);
		} catch (err: unknown) {
			linkError = extractErrorMessage(err, `Failed to unlink ${provider}`);
		} finally {
			unlinkingProvider = null;
		}
	}

	// Social providers configuration
	const socialProviders = [
		{
			id: 'google' as const,
			name: 'Google',
			icon: siGoogle,
			color: `#${siGoogle.hex}`
		},
		{
			id: 'facebook' as const,
			name: 'Facebook',
			icon: siFacebook,
			color: `#${siFacebook.hex}`
		}
	];

	const tabIcons: Record<TabId, typeof User> = {
		profile: User,
		preferences: Settings,
		account: Shield
	};

	const tabIds: TabId[] = ['profile', 'preferences', 'account'];

	function getTabLabel(id: TabId): string {
		return t(`common.${id}`);
	}

	function getRoleBadgeStyle(role: string): { bg: string; text: string } {
		// Returns CSS variable-based colors that adapt to light/dark mode
		switch (role) {
			case 'admin':
				return { bg: 'var(--color-badge-admin-bg)', text: 'var(--color-badge-admin-text)' };
			case 'organizer':
				return { bg: 'var(--color-badge-organizer-bg)', text: 'var(--color-badge-organizer-text)' };
			default:
				return { bg: 'var(--color-badge-user-bg)', text: 'var(--color-badge-user-text)' };
		}
	}
</script>

<svelte:head>
	<title>Account Settings - Loafy Club</title>
</svelte:head>

<PageBackground>
	<Navigation />

	<main class="mx-auto max-w-6xl px-4 py-8 sm:px-6 lg:px-8">
		<AnimatedContainer animation="fade-up">
			<SectionHeader title={t('account.title')} subtitle={t('account.subtitle')} />
		</AnimatedContainer>

		{#if authStore.loading}
			<div class="flex justify-center py-12">
				<LoadingSpinner size="lg" text={t('common.loading')} />
			</div>
		{:else if !authStore.isAuthenticated}
			<AnimatedContainer animation="fade-up" delay={100}>
				<GlassCard class="p-8 text-center">
					<p class="mb-4" style="color: var(--color-body);">{t('account.notAuthenticated')}</p>
					<Button onclick={() => goto('/auth/login')}>{t('common.signIn')}</Button>
				</GlassCard>
			</AnimatedContainer>
		{:else}
			<AnimatedContainer animation="fade-up" delay={100}>
				<div class="grid lg:grid-cols-4 gap-6">
					<!-- Sidebar -->
					<div class="lg:col-span-1">
						<GlassCard class="p-4">
							<nav class="space-y-1">
								{#each tabIds as tabId}
									{@const IconComponent = tabIcons[tabId]}
									<button
										class="w-full flex items-center gap-3 px-4 py-3 rounded-xl text-left transition-all duration-200 {activeTab ===
										tabId
											? 'bg-gradient-to-r from-orange-500 to-pink-500 text-white shadow-lg'
											: ''}"
										style={activeTab !== tabId ? 'color: var(--color-tab-inactive);' : ''}
										onclick={() => (activeTab = tabId)}
									>
										<IconComponent class="w-5 h-5" />
										<span class="font-medium">{getTabLabel(tabId)}</span>
									</button>
								{/each}
							</nav>
						</GlassCard>
					</div>

					<!-- Content -->
					<div class="lg:col-span-3">
						<GlassCard class="p-6 sm:p-8">
							{#if saveSuccess}
								<div
									class="mb-6 p-4 rounded-xl border flex items-center gap-2"
									style="background-color: var(--color-success-bg); border-color: var(--color-success-border); color: var(--color-success-text);"
								>
									<Check class="w-5 h-5" />
									{t('account.messages.saveSuccess')}
								</div>
							{/if}

							{#if saveError}
								<div
									class="mb-6 p-4 rounded-xl border flex items-center gap-2"
									style="background-color: var(--color-error-bg); border-color: var(--color-error-border); color: var(--color-error-text);"
								>
									<X class="w-5 h-5" />
									{saveError}
								</div>
							{/if}
							{#if activeTab === 'profile'}
								<h2
									class="text-2xl font-bold mb-6 font-display"
									style="color: var(--color-heading);"
								>
									{t('account.profile.title')}
								</h2>

								<div class="space-y-6">
									<!-- Avatar placeholder -->
									<div class="flex items-center gap-4">
										<div
											class="w-20 h-20 rounded-full bg-gradient-to-br from-orange-400 to-pink-500 flex items-center justify-center text-white text-2xl font-bold"
										>
											{authStore.user?.name?.[0]?.toUpperCase() ||
												authStore.supabaseUser?.email?.[0]?.toUpperCase() ||
												'?'}
										</div>
										<div>
											<p class="text-sm" style="color: var(--color-body);">{t('account.profile.photo')}</p>
											<p class="text-xs" style="color: var(--color-subtle);">{t('account.profile.photoComingSoon')}</p>
										</div>
									</div>

									<!-- Name -->
									<div class="space-y-2">
										<Label for="name">{t('account.profile.fullName')}</Label>
										<Input
											id="name"
											type="text"
											placeholder={t('account.profile.namePlaceholder')}
											bind:value={name}
											class="max-w-md"
										/>
									</div>

									<!-- Phone -->
									<div class="space-y-2">
										<Label for="phone">{t('account.profile.phone')}</Label>
										<Input
											id="phone"
											type="tel"
											placeholder={t('account.profile.phonePlaceholder')}
											bind:value={phone}
											class="max-w-md"
										/>
									</div>

									<!-- Email (read-only) -->
									<div class="space-y-2">
										<Label for="email">{t('account.profile.email')}</Label>
										<Input
											id="email"
											type="email"
											value={authStore.supabaseUser?.email || ''}
											disabled
											class="max-w-md" style="background-color: var(--color-muted);"
										/>
										<p class="text-xs" style="color: var(--color-subtle);">{t('account.profile.emailNote')}</p>
									</div>

									<!-- Role badge -->
									<div class="space-y-2">
										<Label>{t('account.profile.role')}</Label>
										<div>
											<span
												class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium capitalize"
												style="background-color: {getRoleBadgeStyle(authStore.user?.role || 'user').bg}; color: {getRoleBadgeStyle(authStore.user?.role || 'user').text};"
											>
												{authStore.user?.role || 'user'}
											</span>
										</div>
									</div>

									<!-- Save button -->
									<div class="pt-4">
										<Button
											variant="gradient"
											onclick={saveProfile}
											disabled={saving}
										>
											{#if saving}
												<LoadingSpinner size="sm" class="mr-2" />
												{t('account.profile.saving')}
											{:else}
												{t('account.profile.saveChanges')}
											{/if}
										</Button>
									</div>
								</div>
							{:else if activeTab === 'preferences'}
								<h2
									class="text-2xl font-bold mb-6 font-display"
									style="color: var(--color-heading);"
								>
									{t('account.preferences.title')}
								</h2>

								<div class="space-y-6">
									<!-- Language -->
									<div class="space-y-2">
										<Label for="language">{t('account.preferences.language')}</Label>
										<SelectNative
											id="language"
											value={i18n.locale}
											onchange={(e) => i18n.setLocale(e.currentTarget.value as Locale)}
											variant="styled"
											class="max-w-md"
											style="background-color: var(--color-card); border-color: var(--color-border); color: var(--color-foreground);"
										>
											<option value="en">{t('languages.en')}</option>
											<option value="vi">{t('languages.vi')}</option>
										</SelectNative>
										<p class="text-xs" style="color: var(--color-subtle);">{t('account.preferences.languageNote')}</p>
									</div>

									<!-- Theme -->
									<div class="space-y-2">
										<Label for="theme">{t('account.preferences.theme')}</Label>
										<SelectNative
											id="theme"
											value={theme}
											onchange={(e) => setTheme(e.currentTarget.value as Theme)}
											variant="styled"
											class="max-w-md"
											style="background-color: var(--color-card); border-color: var(--color-border); color: var(--color-foreground);"
										>
											<option value="system">{t('account.preferences.themeSystem')}</option>
											<option value="light">{t('account.preferences.themeLight')}</option>
											<option value="dark">{t('account.preferences.themeDark')}</option>
										</SelectNative>
										<p class="text-xs" style="color: var(--color-subtle);">{t('account.preferences.themeNote', { mode: themeStore.resolvedTheme })}</p>
									</div>

									<!-- Email Notifications -->
									<div class="space-y-2">
										<Label>{t('account.preferences.emailNotifications')}</Label>
										<label class="flex items-center gap-3 cursor-pointer">
											<input
												type="checkbox"
												bind:checked={emailNotifications}
												class="w-5 h-5 rounded text-orange-500 focus:ring-orange-500" style="border-color: var(--color-border);"
											/>
											<span class="text-sm" style="color: var(--color-body);"
												>{t('account.preferences.emailNotificationsLabel')}</span
											>
										</label>
										<p class="text-xs" style="color: var(--color-subtle);">{t('account.preferences.emailNotificationsNote')}</p>
									</div>

									<!-- Save button -->
									<div class="pt-4">
										<Button
											variant="gradient"
											onclick={savePreferences}
										>
											{t('account.preferences.savePreferences')}
										</Button>
									</div>
								</div>
							{:else if activeTab === 'account'}
								<h2
									class="text-2xl font-bold mb-6 font-display"
									style="color: var(--color-heading);"
								>
									{t('account.actions.title')}
								</h2>

								<div class="space-y-6">
									<!-- Connected Accounts -->
									<div class="space-y-3">
										<Label>{t('account.connected.title')}</Label>
										<p class="text-sm" style="color: var(--color-subtle);">{t('account.connected.description')}</p>

										{#if linkError}
											<div
												class="p-3 rounded-xl border flex items-center gap-2 text-sm"
												style="background-color: var(--color-error-bg); border-color: var(--color-error-border); color: var(--color-error-text);"
											>
												<X class="w-4 h-4" />
												{linkError}
											</div>
										{/if}

										<div class="space-y-2">
											{#each socialProviders as provider}
												{@const isConnected = authStore.isProviderConnected(provider.id)}
												{@const isLinking = linkingProvider === provider.id}
												{@const isUnlinking = unlinkingProvider === provider.id}
												<div
													class="p-4 rounded-xl border flex items-center justify-between"
													style="background-color: var(--color-muted); border-color: var(--color-border);"
												>
													<div class="flex items-center gap-3">
														<svg class="w-5 h-5" viewBox="0 0 24 24" fill={provider.color}>
															{@html provider.icon.svg}
														</svg>
														<div>
															<p class="font-medium" style="color: var(--color-heading);">{provider.name}</p>
															<p class="text-xs" style="color: var(--color-subtle);">
																{#if isConnected}
																	{t('account.connected.connected')}
																{:else}
																	{t('account.connected.notConnected')}
																{/if}
															</p>
														</div>
													</div>
													{#if isConnected}
														<Button
															variant="outline"
															size="sm"
															onclick={() => handleUnlinkProvider(provider.id)}
															disabled={isUnlinking || authStore.identities.length <= 1}
														>
															{#if isUnlinking}
																<LoadingSpinner size="sm" class="mr-2" />
															{:else}
																<Unlink2 class="w-4 h-4 mr-2" />
															{/if}
															{t('account.connected.unlink')}
														</Button>
													{:else}
														<Button
															variant="outline"
															size="sm"
															onclick={() => handleLinkProvider(provider.id)}
															disabled={isLinking}
														>
															{#if isLinking}
																<LoadingSpinner size="sm" class="mr-2" />
															{:else}
																<Link2 class="w-4 h-4 mr-2" />
															{/if}
															{t('account.connected.link')}
														</Button>
													{/if}
												</div>
											{/each}
										</div>

										{#if authStore.identities.length <= 1}
											<p class="text-xs" style="color: var(--color-subtle);">
												{t('account.connected.mustHaveOne')}
											</p>
										{/if}
									</div>

									<div class="border-t pt-6" style="border-color: var(--color-border);"></div>

									<!-- Sign Out -->
									<div
										class="p-4 rounded-xl border flex items-center justify-between"
										style="background-color: var(--color-muted); border-color: var(--color-border);"
									>
										<div>
											<h3 class="font-medium" style="color: var(--color-heading);">{t('account.actions.signOutTitle')}</h3>
											<p class="text-sm" style="color: var(--color-body);">{t('account.actions.signOutDescription')}</p>
										</div>
										<Button variant="outline" onclick={handleSignOut}>{t('common.signOut')}</Button>
									</div>

									<!-- Delete Account -->
									<div
										class="p-4 rounded-xl border"
										style="background-color: var(--color-error-bg); border-color: var(--color-error-border);"
									>
										{#if !showDeleteConfirm}
											<div class="flex items-center justify-between">
												<div>
													<h3 class="font-medium" style="color: var(--color-error-text);">{t('account.actions.deleteTitle')}</h3>
													<p class="text-sm" style="color: var(--color-error-text); opacity: 0.8;">
														{t('account.actions.deleteDescription')}
													</p>
												</div>
												<Button variant="destructive" onclick={() => showDeleteConfirm = true}>
													{t('common.delete')}
												</Button>
											</div>
										{:else}
											<div class="space-y-4">
												<div>
													<h3 class="font-medium" style="color: var(--color-error-text);">{t('account.delete.confirmTitle')}</h3>
													<p class="text-sm mt-1" style="color: var(--color-error-text); opacity: 0.8;">
														{t('account.delete.confirmDescription')}
													</p>
												</div>

												{#if deleteError}
													<div
														class="p-3 rounded-lg border flex items-center gap-2 text-sm"
														style="background-color: var(--color-card); border-color: var(--color-error-border); color: var(--color-error-text);"
													>
														<X class="w-4 h-4" />
														{deleteError}
													</div>
												{/if}

												<div class="space-y-2">
													<Label for="deleteConfirm" class="text-sm" style="color: var(--color-error-text);">
														{t('account.delete.typeConfirm')}
													</Label>
													<Input
														id="deleteConfirm"
														type="text"
														placeholder="DELETE"
														bind:value={deleteConfirmText}
														class="max-w-xs"
														style="border-color: var(--color-error-border);"
													/>
												</div>

												<div class="flex gap-3">
													<Button
														variant="destructive"
														onclick={handleDeleteAccount}
														disabled={deleting || deleteConfirmText !== 'DELETE'}
													>
														{#if deleting}
															<LoadingSpinner size="sm" class="mr-2" />
															{t('account.delete.deleting')}
														{:else}
															{t('account.delete.confirmButton')}
														{/if}
													</Button>
													<Button variant="outline" onclick={cancelDelete} disabled={deleting}>
														{t('common.cancel')}
													</Button>
												</div>
											</div>
										{/if}
									</div>
								</div>
							{/if}
						</GlassCard>
					</div>
				</div>
			</AnimatedContainer>
		{/if}
	</main>
</PageBackground>
