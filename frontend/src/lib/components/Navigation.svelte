<script lang="ts">
	import { authStore } from '$lib/stores/auth.svelte';
	import { goto } from '$app/navigation';
	import { Button } from '$lib/components/ui/button';
	import { useTranslation } from '$lib/i18n/index.svelte';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';

	interface Props {
		hidden?: boolean;
		noSpacer?: boolean;
	}

	let { hidden = false, noSpacer = false }: Props = $props();

	const t = useTranslation();

	async function handleSignOut() {
		await authStore.signOut();
		goto('/auth/login');
	}
</script>

<nav
	class="fixed top-0 left-0 right-0 z-50 border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 transition-all duration-300 {hidden ? '-translate-y-full opacity-0' : 'translate-y-0 opacity-100'}"
>
	<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
		<div class="flex h-16 items-center justify-between">
			<div class="flex items-center gap-8">
				<a href="/" class="flex items-center gap-2 text-2xl font-bold text-primary hover:opacity-90 transition-opacity group">
					<img src="/mascot.png" alt="Loafy" class="h-10 w-10 object-contain group-hover:scale-110 transition-transform duration-300" />
					<span class="text-gradient-loafy font-display">Loafy Club</span>
				</a>
				<div class="hidden md:flex items-center gap-6">
					<a href="/sessions" class="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors">
						{t('nav.sessions')}
					</a>
				</div>
			</div>

			<div class="flex items-center gap-4">
				{#if authStore.isAuthenticated}
					<DropdownMenu.Root>
						<DropdownMenu.Trigger class="flex items-center gap-2 rounded-full pr-1 pl-1 py-1 hover:bg-muted transition-colors cursor-pointer focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2">
							<div class="w-8 h-8 rounded-full bg-gradient-to-br from-orange-400 to-pink-500 flex items-center justify-center text-white text-xs font-bold shrink-0">
								{authStore.user?.name?.[0]?.toUpperCase() || authStore.supabaseUser?.email?.[0]?.toUpperCase() || '?'}
							</div>
							<span class="hidden sm:inline text-sm font-medium text-foreground pr-2">
								{authStore.user?.name || authStore.supabaseUser?.email?.split('@')[0]}
							</span>
							<svg class="w-4 h-4 text-muted-foreground hidden sm:block" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
							</svg>
						</DropdownMenu.Trigger>
						<DropdownMenu.Content class="w-56" align="end">
							<DropdownMenu.Label class="font-normal">
								<div class="flex flex-col space-y-1">
									<p class="text-sm font-medium leading-none">{authStore.user?.name || authStore.supabaseUser?.email?.split('@')[0]}</p>
									<p class="text-xs leading-none text-muted-foreground">{authStore.supabaseUser?.email}</p>
								</div>
							</DropdownMenu.Label>
							<DropdownMenu.Separator />
							<DropdownMenu.Group>
								<DropdownMenu.Item onclick={() => goto('/bookings')}>
									<svg class="mr-2 h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
									</svg>
									{t('nav.myBookings')}
								</DropdownMenu.Item>
								<DropdownMenu.Item onclick={() => goto('/account')}>
									<svg class="mr-2 h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
									</svg>
									{t('common.settings')}
								</DropdownMenu.Item>
							</DropdownMenu.Group>

							{#if authStore.isOrganizer || authStore.isAdmin}
								<DropdownMenu.Separator />
								<DropdownMenu.Group>
									{#if authStore.isOrganizer}
										<DropdownMenu.Item onclick={() => goto('/organizer/sessions/create')}>
											<svg class="mr-2 h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
												<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
											</svg>
											{t('nav.createSession')}
										</DropdownMenu.Item>
									{/if}
									{#if authStore.isAdmin}
										<DropdownMenu.Item onclick={() => goto('/admin/sessions')}>
											<svg class="mr-2 h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
												<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
											</svg>
											{t('nav.admin')}
										</DropdownMenu.Item>
									{/if}
								</DropdownMenu.Group>
							{/if}

							<DropdownMenu.Separator />
							<DropdownMenu.Item onclick={handleSignOut} class="text-destructive focus:text-destructive">
								<svg class="mr-2 h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
								</svg>
								{t('common.signOut')}
							</DropdownMenu.Item>
						</DropdownMenu.Content>
					</DropdownMenu.Root>
				{:else}
					<Button onclick={() => goto('/auth/login')}>{t('common.signIn')}</Button>
				{/if}
			</div>
		</div>
	</div>
</nav>

<!-- Spacer to prevent content from hiding behind fixed navbar -->
{#if !noSpacer}
	<div class="h-16"></div>
{/if}
