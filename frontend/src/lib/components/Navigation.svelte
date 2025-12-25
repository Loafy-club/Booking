<script lang="ts">
	import { authStore } from '$lib/stores/auth.svelte';
	import { goto } from '$app/navigation';
	import { Button } from '$lib/components/ui/button';
	import { useTranslation } from '$lib/i18n/index.svelte';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Avatar from '$lib/components/ui/avatar';
	import { Calendar, Settings, Plus, Shield, LogOut, ChevronDown } from 'lucide-svelte';

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
						<DropdownMenu.Trigger class="group flex items-center gap-2 rounded-full pr-1 pl-1 py-1 hover:bg-muted transition-colors cursor-pointer focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2">
							<Avatar.Root class="size-8">
								<Avatar.Image
									src={authStore.user?.avatar_url || authStore.supabaseUser?.user_metadata?.avatar_url || authStore.supabaseUser?.user_metadata?.picture}
									alt={authStore.user?.name || authStore.supabaseUser?.email || 'User'}
								/>
								<Avatar.Fallback class="bg-gradient-to-br from-orange-400 to-pink-500 text-white text-xs font-bold">
									{authStore.user?.name?.[0]?.toUpperCase() || authStore.supabaseUser?.email?.[0]?.toUpperCase() || '?'}
								</Avatar.Fallback>
							</Avatar.Root>
							<span class="hidden sm:inline text-sm font-medium text-foreground pr-2">
								{authStore.user?.name || authStore.supabaseUser?.email?.split('@')[0]}
							</span>
							<ChevronDown class="size-4 text-muted-foreground hidden sm:block transition-transform duration-200 group-data-[state=open]:rotate-180" />
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
									<Calendar class="mr-2 size-4" />
									{t('nav.myBookings')}
								</DropdownMenu.Item>
								<DropdownMenu.Item onclick={() => goto('/account')}>
									<Settings class="mr-2 size-4" />
									{t('common.settings')}
								</DropdownMenu.Item>
							</DropdownMenu.Group>

							{#if authStore.isOrganizer || authStore.isAdmin}
								<DropdownMenu.Separator />
								<DropdownMenu.Group>
									{#if authStore.isOrganizer}
										<DropdownMenu.Item onclick={() => goto('/admin/sessions?action=create')}>
											<Plus class="mr-2 size-4" />
											{t('nav.createSession')}
										</DropdownMenu.Item>
									{/if}
									{#if authStore.isAdmin}
										<DropdownMenu.Item onclick={() => goto('/admin')}>
											<Shield class="mr-2 size-4" />
											{t('nav.admin')}
										</DropdownMenu.Item>
									{/if}
								</DropdownMenu.Group>
							{/if}

							<DropdownMenu.Separator />
							<DropdownMenu.Item onclick={handleSignOut} class="text-destructive focus:text-destructive">
								<LogOut class="mr-2 size-4" />
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
