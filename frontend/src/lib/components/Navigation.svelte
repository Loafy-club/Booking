<script lang="ts">
	import { authStore } from '$lib/stores/auth.svelte';
	import { goto } from '$app/navigation';
	import Button from '$lib/components/ui/Button.svelte';

	async function handleSignOut() {
		await authStore.signOut();
		goto('/auth/login');
	}
</script>

<nav class="border-b bg-white">
	<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
		<div class="flex h-16 items-center justify-between">
			<div class="flex items-center">
				<a href="/" class="text-2xl font-bold text-primary">Loafy Club</a>
			</div>

			<div class="flex items-center gap-4">
				{#if authStore.isAuthenticated}
					<a href="/sessions" class="text-sm font-medium text-gray-700 hover:text-primary">
						Sessions
					</a>
					<a href="/bookings" class="text-sm font-medium text-gray-700 hover:text-primary">
						My Bookings
					</a>

					{#if authStore.isOrganizer}
						<a
							href="/organizer/sessions/create"
							class="text-sm font-medium text-gray-700 hover:text-primary"
						>
							Create Session
						</a>
					{/if}

					{#if authStore.isAdmin}
						<a href="/admin/sessions" class="text-sm font-medium text-gray-700 hover:text-primary">
							Admin
						</a>
					{/if}

					<div class="flex items-center gap-2 border-l pl-4">
						<span class="text-sm text-gray-600">{authStore.user?.email}</span>
						<Button variant="outline" size="sm" onclick={handleSignOut}>Sign Out</Button>
					</div>
				{:else}
					<Button onclick={() => goto('/auth/login')}>Sign In</Button>
				{/if}
			</div>
		</div>
	</div>
</nav>
