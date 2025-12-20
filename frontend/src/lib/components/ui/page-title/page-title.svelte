<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
		subtitle?: string;
		centered?: boolean;
		class?: string;
	}

	let { children, subtitle, centered = true, class: className = '' }: Props = $props();
</script>

<div class="{centered ? 'text-center' : ''} {className}">
	<h1
		class="text-4xl sm:text-5xl font-bold text-gray-800"
		style="font-family: 'Baloo 2', sans-serif;"
	>
		{@render children()}
	</h1>
	{#if subtitle}
		<p class="mt-4 text-lg text-gray-600">{subtitle}</p>
	{/if}
</div>
