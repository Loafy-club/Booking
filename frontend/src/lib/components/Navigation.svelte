<script lang="ts">
	import { authStore } from '$lib/stores/auth.svelte';
	import { goto } from '$app/navigation';
	import { Button } from '$lib/components/ui/button';

	interface Props {
		hidden?: boolean;
		noSpacer?: boolean;
	}

	let { hidden = false, noSpacer = false }: Props = $props();

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
					<span class="text-gradient-loafy" style="font-family: 'Baloo 2', sans-serif;">Loafy Club</span>
				</a>
				<div class="hidden md:flex items-center gap-6">
					<a href="/sessions" class="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors">
						Sessions
					</a>
				</div>
			</div>

			<div class="flex items-center gap-4">
				{#if authStore.isAuthenticated}
					<a href="/bookings" class="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors">
						My Bookings
					</a>

					{#if authStore.isOrganizer}
						<a
							href="/organizer/sessions/create"
							class="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors"
						>
							Create Session
						</a>
					{/if}

					{#if authStore.isAdmin}
						<a href="/admin/sessions" class="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors">
							Admin
						</a>
					{/if}

					<div class="flex items-center gap-3 border-l pl-4">
						<span class="text-sm text-muted-foreground">{authStore.user?.email}</span>
						<Button variant="outline" size="sm" onclick={handleSignOut}>Sign Out</Button>
					</div>
				{:else}
					<Button onclick={() => goto('/auth/login')}>Sign In</Button>
				{/if}
			</div>
		</div>
	</div>
</nav>

<!-- Spacer to prevent content from hiding behind fixed navbar -->
{#if !noSpacer}
	<div class="h-16"></div>
{/if}
