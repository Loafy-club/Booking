<script lang="ts">
	import Navigation from '$lib/components/Navigation.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { PageBackground } from '$lib/components/ui/page-background';
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
		variant?: 'subtle' | 'default';
		class?: string;
	}

	let { children, variant = 'subtle', class: className = '' }: Props = $props();
</script>

<PageBackground {variant} noFooter>
	<div class="flex flex-col min-h-screen">
		<Navigation />

		<main class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8 flex-1 w-full {className}">
			{@render children()}
		</main>

		<Footer />
	</div>
</PageBackground>
