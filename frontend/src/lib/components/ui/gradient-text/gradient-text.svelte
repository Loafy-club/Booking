<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
		variant?: 'default' | 'hero' | 'subtle';
		as?: 'span' | 'h1' | 'h2' | 'h3' | 'h4' | 'p';
		class?: string;
	}

	let { children, variant = 'default', as = 'span', class: className = '' }: Props = $props();

	const gradients = {
		default: 'from-orange-500 to-pink-500',
		hero: 'from-orange-400 via-pink-500 to-rose-500',
		subtle: 'from-orange-400 to-orange-600'
	};
</script>

{#if as === 'h1'}
	<h1
		class="text-transparent bg-clip-text bg-gradient-to-r {gradients[variant]} {className}"
		style="font-family: 'Baloo 2', sans-serif;"
	>
		{@render children()}
	</h1>
{:else if as === 'h2'}
	<h2
		class="text-transparent bg-clip-text bg-gradient-to-r {gradients[variant]} {className}"
		style="font-family: 'Baloo 2', sans-serif;"
	>
		{@render children()}
	</h2>
{:else if as === 'h3'}
	<h3
		class="text-transparent bg-clip-text bg-gradient-to-r {gradients[variant]} {className}"
		style="font-family: 'Baloo 2', sans-serif;"
	>
		{@render children()}
	</h3>
{:else if as === 'h4'}
	<h4
		class="text-transparent bg-clip-text bg-gradient-to-r {gradients[variant]} {className}"
		style="font-family: 'Baloo 2', sans-serif;"
	>
		{@render children()}
	</h4>
{:else if as === 'p'}
	<p
		class="text-transparent bg-clip-text bg-gradient-to-r {gradients[variant]} {className}"
		style="font-family: 'Baloo 2', sans-serif;"
	>
		{@render children()}
	</p>
{:else}
	<span
		class="text-transparent bg-clip-text bg-gradient-to-r {gradients[variant]} {className}"
		style="font-family: 'Baloo 2', sans-serif;"
	>
		{@render children()}
	</span>
{/if}
