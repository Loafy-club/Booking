<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { authStore } from '$lib/stores/auth.svelte';
	import { themeStore } from '$lib/stores/theme.svelte';
	import { useTranslation } from '$lib/i18n/index.svelte';
	import { LanguageSelector } from '$lib/components/ui/language-selector';
	import { api } from '$lib/api/client';
	import { extractErrorMessage } from '$lib/utils';
	import { getRoleBadgeVariant } from '$lib/utils/status';
	import Navigation from '$lib/components/Navigation.svelte';
	import { PageBackground } from '$lib/components/ui/page-background';
	import { Card } from '$lib/components/ui/card';
	import { SectionHeader } from '$lib/components/ui/section-header';
	import { AnimatedContainer } from '$lib/components/ui/animated-container';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import * as Select from '$lib/components/ui/select';
	import { Switch } from '$lib/components/ui/switch';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { Spinner } from '$lib/components/ui/spinner';
	import { AccountSkeleton } from '$lib/components/ui/skeleton';
	import * as Alert from '$lib/components/ui/alert';
	import * as Avatar from '$lib/components/ui/avatar';
	import { Badge } from '$lib/components/ui/badge';
	import * as Tabs from '$lib/components/ui/tabs';
	import { Check, X, User, Settings, Shield, Link2, Unlink2, Sun, Moon } from 'lucide-svelte';
	import { siGoogle, siFacebook } from 'simple-icons';

	type TabId = 'profile' | 'preferences' | 'account';
	type Theme = 'light' | 'dark';

	let activeTab = $state<TabId | undefined>('profile');
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
	let showSkeleton = $state(false);

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

		// Show skeleton only if loading takes > 200ms
		const skeletonTimer = setTimeout(() => {
			if (authStore.loading) showSkeleton = true;
		}, 200);

		// Load email notifications preference
		const savedNotifications = localStorage.getItem('loafy_email_notifications');
		if (savedNotifications !== null) emailNotifications = savedNotifications === 'true';

		return () => clearTimeout(skeletonTimer);
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

		{#if authStore.loading && showSkeleton}
			<AccountSkeleton />
		{:else if authStore.loading}
			<!-- Brief loading -->
		{:else if !authStore.isAuthenticated}
			<AnimatedContainer animation="fade-up" delay={100}>
				<Card variant="glass" class="p-8 text-center">
					<p class="mb-4 text-muted-foreground">{t('account.notAuthenticated')}</p>
					<Button onclick={() => goto('/auth/login')}>{t('common.signIn')}</Button>
				</Card>
			</AnimatedContainer>
		{:else}
			<AnimatedContainer animation="fade-up" delay={100}>
				<Tabs.Root bind:value={activeTab} class="grid lg:grid-cols-4 gap-6" orientation="vertical">
					<!-- Sidebar -->
					<div class="lg:col-span-1">
						<Card variant="glass" class="p-6">
							<Tabs.List class="flex flex-col w-full h-auto bg-transparent p-0 gap-1">
								{#each tabIds as tabId}
									{@const IconComponent = tabIcons[tabId]}
									<Tabs.Trigger
										value={tabId}
										class="w-full justify-start gap-3 px-4 py-3 rounded-xl text-left transition-all duration-200 h-auto data-[state=active]:bg-gradient-to-r data-[state=active]:from-orange-500 data-[state=active]:to-pink-500 data-[state=active]:text-white data-[state=active]:shadow-lg data-[state=inactive]:text-muted-foreground data-[state=inactive]:hover:text-foreground data-[state=inactive]:bg-transparent data-[state=active]:border-transparent"
									>
										<IconComponent class="w-5 h-5" />
										<span class="font-medium">{getTabLabel(tabId)}</span>
									</Tabs.Trigger>
								{/each}
							</Tabs.List>
						</Card>
					</div>

					<!-- Content -->
					<div class="lg:col-span-3">
						<Card variant="glass" class="p-6">
							{#if saveSuccess}
								<Alert.Root variant="success" class="mb-6">
									<Check class="size-4" />
									<Alert.Description>{t('account.messages.saveSuccess')}</Alert.Description>
								</Alert.Root>
							{/if}

							{#if saveError}
								<Alert.Root variant="destructive" class="mb-6">
									<X class="size-4" />
									<Alert.Description>{saveError}</Alert.Description>
								</Alert.Root>
							{/if}
							<Tabs.Content value="profile">
								<h2
									class="text-2xl font-bold mb-6 font-display text-foreground"
								>
									{t('account.profile.title')}
								</h2>

								<div class="space-y-6">
									<!-- Avatar placeholder -->
									<div class="flex items-center gap-4">
										<Avatar.Root class="size-20">
											<Avatar.Image
												src={authStore.user?.avatar_url || authStore.supabaseUser?.user_metadata?.avatar_url || authStore.supabaseUser?.user_metadata?.picture}
												alt={authStore.user?.name || authStore.supabaseUser?.email || 'User'}
											/>
											<Avatar.Fallback class="bg-gradient-to-br from-orange-400 to-pink-500 text-white text-2xl font-bold">
												{authStore.user?.name?.[0]?.toUpperCase() ||
													authStore.supabaseUser?.email?.[0]?.toUpperCase() ||
													'?'}
											</Avatar.Fallback>
										</Avatar.Root>
										<div>
											<p class="text-sm text-muted-foreground">{t('account.profile.photo')}</p>
											<p class="text-xs text-muted-foreground">{t('account.profile.photoComingSoon')}</p>
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
											class="max-w-md bg-muted"
										/>
										<p class="text-xs text-muted-foreground">{t('account.profile.emailNote')}</p>
									</div>

									<!-- Role badge -->
									<div class="space-y-2">
										<Label>{t('account.profile.role')}</Label>
										<div>
											<Badge variant={getRoleBadgeVariant(authStore.user?.role || 'user')} size="lg" class="capitalize rounded-full">
												{authStore.user?.role || 'user'}
											</Badge>
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
												<Spinner class="size-4 mr-2" />
												{t('account.profile.saving')}
											{:else}
												{t('account.profile.saveChanges')}
											{/if}
										</Button>
									</div>
								</div>
							</Tabs.Content>
							<Tabs.Content value="preferences">
								<h2
									class="text-2xl font-bold mb-6 font-display text-foreground"
								>
									{t('account.preferences.title')}
								</h2>

								<div class="space-y-6">
									<!-- Language -->
									<div class="space-y-2">
										<Label>{t('account.preferences.language')}</Label>
										<LanguageSelector class="max-w-md" />
										<p class="text-xs text-muted-foreground">{t('account.preferences.languageNote')}</p>
									</div>

									<!-- Theme -->
									<div class="space-y-2">
										<Label>{t('account.preferences.theme')}</Label>
										<Select.Root type="single" value={theme} onValueChange={(value) => value && setTheme(value as Theme)}>
											<Select.Trigger class="max-w-md">
												<Select.Value placeholder={t('account.preferences.selectTheme')}>
													{#if theme === 'light'}
														<span class="flex items-center gap-2"><Sun class="size-4" /> {t('account.preferences.themeLight')}</span>
													{:else if theme === 'dark'}
														<span class="flex items-center gap-2"><Moon class="size-4" /> {t('account.preferences.themeDark')}</span>
													{/if}
												</Select.Value>
											</Select.Trigger>
											<Select.Content>
												<Select.Item value="light"><span class="flex items-center gap-2"><Sun class="size-4" /> {t('account.preferences.themeLight')}</span></Select.Item>
												<Select.Item value="dark"><span class="flex items-center gap-2"><Moon class="size-4" /> {t('account.preferences.themeDark')}</span></Select.Item>
											</Select.Content>
										</Select.Root>
									</div>

									<!-- Email Notifications -->
									<div class="space-y-2">
										<Label>{t('account.preferences.emailNotifications')}</Label>
										<div class="flex items-center gap-3">
											<Switch
												bind:checked={emailNotifications}
												class="data-[state=checked]:bg-orange-500"
											/>
											<span class="text-sm text-muted-foreground"
												>{t('account.preferences.emailNotificationsLabel')}</span
											>
										</div>
										<p class="text-xs text-muted-foreground">{t('account.preferences.emailNotificationsNote')}</p>
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
							</Tabs.Content>
							<Tabs.Content value="account">
								<h2
									class="text-2xl font-bold mb-6 font-display text-foreground"
								>
									{t('account.actions.title')}
								</h2>

								<div class="space-y-8">
									<!-- Connected Accounts -->
									<div class="space-y-4">
										<div class="space-y-1">
											<Label>{t('account.connected.title')}</Label>
											<p class="text-sm text-muted-foreground">{t('account.connected.description')}</p>
										</div>

										{#if linkError}
											<Alert.Root variant="destructive">
												<X class="size-4" />
												<Alert.Description>{linkError}</Alert.Description>
											</Alert.Root>
										{/if}

										<div class="space-y-3">
											{#each socialProviders as provider}
												{@const isConnected = authStore.isProviderConnected(provider.id)}
												{@const isLinking = linkingProvider === provider.id}
												{@const isUnlinking = unlinkingProvider === provider.id}
												<div
													class="p-4 rounded-xl border flex items-center justify-between bg-muted border-border"
												>
													<div class="flex items-center gap-3">
														<svg class="w-5 h-5" viewBox="0 0 24 24" fill={provider.color}>
															{@html provider.icon.svg}
														</svg>
														<div>
															<p class="font-medium text-foreground">{provider.name}</p>
															<p class="text-xs text-muted-foreground">
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
																<Spinner class="size-4 mr-2" />
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
																<Spinner class="size-4 mr-2" />
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
											<p class="text-xs text-muted-foreground">
												{t('account.connected.mustHaveOne')}
											</p>
										{/if}
									</div>

									<div class="border-t border-border"></div>

									<!-- Actions Section -->
									<div class="space-y-3">
										<!-- Sign Out -->
										<div
											class="p-4 rounded-xl border flex items-center justify-between bg-muted border-border"
										>
											<div>
												<h3 class="font-medium text-foreground">{t('account.actions.signOutTitle')}</h3>
												<p class="text-sm text-muted-foreground">{t('account.actions.signOutDescription')}</p>
											</div>
											<Button variant="outline" onclick={handleSignOut}>{t('common.signOut')}</Button>
										</div>

										<!-- Delete Account -->
										<div
											class="p-4 rounded-xl border bg-error-bg border-error-border"
										>
										<div class="flex items-center justify-between">
											<div>
												<h3 class="font-medium text-error-text">{t('account.actions.deleteTitle')}</h3>
												<p class="text-sm text-error-text/80">
													{t('account.actions.deleteDescription')}
												</p>
											</div>
											<AlertDialog.Root bind:open={showDeleteConfirm} onOpenChange={(open) => { if (!open) cancelDelete(); }}>
												<AlertDialog.Trigger>
													{#snippet child({ props })}
														<Button variant="destructive" {...props}>
															{t('common.delete')}
														</Button>
													{/snippet}
												</AlertDialog.Trigger>
												<AlertDialog.Content class="sm:max-w-md">
													<AlertDialog.Header>
														<AlertDialog.Title class="text-destructive">
															{t('account.delete.confirmTitle')}
														</AlertDialog.Title>
														<AlertDialog.Description>
															{t('account.delete.confirmDescription')}
														</AlertDialog.Description>
													</AlertDialog.Header>

													{#if deleteError}
														<Alert.Root variant="destructive">
															<X class="size-4" />
															<Alert.Description>{deleteError}</Alert.Description>
														</Alert.Root>
													{/if}

													<div class="space-y-2">
														<Label for="deleteConfirm" class="text-sm text-destructive">
															{t('account.delete.typeConfirm')}
														</Label>
														<Input
															id="deleteConfirm"
															type="text"
															placeholder="DELETE"
															bind:value={deleteConfirmText}
															class="border-destructive"
														/>
													</div>

													<AlertDialog.Footer>
														<AlertDialog.Cancel disabled={deleting}>
															{t('common.cancel')}
														</AlertDialog.Cancel>
														<Button
															variant="destructive"
															onclick={handleDeleteAccount}
															disabled={deleting || deleteConfirmText !== 'DELETE'}
														>
															{#if deleting}
																<Spinner class="size-4 mr-2" />
																{t('account.delete.deleting')}
															{:else}
																{t('account.delete.confirmButton')}
															{/if}
														</Button>
													</AlertDialog.Footer>
												</AlertDialog.Content>
											</AlertDialog.Root>
										</div>
									</div>
									</div>
								</div>
							</Tabs.Content>
						</Card>
					</div>
				</Tabs.Root>
			</AnimatedContainer>
		{/if}
	</main>
</PageBackground>
