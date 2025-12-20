<script lang="ts">
	import { authStore } from '$lib/stores/auth.svelte';
	import { goto } from '$app/navigation';
	import Navigation from '$lib/components/Navigation.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Card from '$lib/components/ui/Card.svelte';
</script>

<svelte:head>
	<title>Home - Loafy Club</title>
</svelte:head>

<Navigation />

<div class="mx-auto max-w-7xl px-4 py-12 sm:px-6 lg:px-8">
	<div class="mb-12 text-center">
		<h1 class="text-5xl font-bold text-gray-900">Welcome to Loafy Club</h1>
		<p class="mt-4 text-xl text-gray-600">
			Book your pickleball sessions in Hanoi with ease
		</p>

		{#if !authStore.isAuthenticated}
			<div class="mt-8">
				<Button size="lg" onclick={() => goto('/auth/login')}>Get Started</Button>
			</div>
		{/if}
	</div>

	<div class="grid gap-8 md:grid-cols-3">
		<Card class="p-6">
			<h3 class="text-xl font-semibold text-gray-900">Browse Sessions</h3>
			<p class="mt-2 text-gray-600">
				Find available pickleball sessions that fit your schedule
			</p>
			{#if authStore.isAuthenticated}
				<Button class="mt-4" variant="outline" onclick={() => goto('/sessions')}>
					View Sessions
				</Button>
			{/if}
		</Card>

		<Card class="p-6">
			<h3 class="text-xl font-semibold text-gray-900">Easy Booking</h3>
			<p class="mt-2 text-gray-600">
				Book your spot in seconds with our streamlined booking process
			</p>
		</Card>

		<Card class="p-6">
			<h3 class="text-xl font-semibold text-gray-900">Flexible Payment</h3>
			<p class="mt-2 text-gray-600">
				Pay with Stripe or local QR payment methods
			</p>
		</Card>
	</div>

	{#if authStore.isAuthenticated && authStore.user}
		<div class="mt-12 rounded-lg bg-primary/10 p-6">
			<h2 class="text-2xl font-semibold">Welcome back, {authStore.user.full_name || authStore.user.email}!</h2>
			<p class="mt-2 text-gray-600">Ready to book your next pickleball session?</p>
			<div class="mt-4 flex gap-4">
				<Button onclick={() => goto('/sessions')}>Browse Sessions</Button>
				<Button variant="outline" onclick={() => goto('/bookings')}>My Bookings</Button>
			</div>
		</div>
	{/if}
</div>
