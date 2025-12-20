<script lang="ts">
	import type { Snippet } from 'svelte';
	import { GradientText } from '$lib/components/ui/gradient-text';

	interface Props {
		title: string;
		subtitle?: string;
		gradient?: boolean;
		centered?: boolean;
		actions?: Snippet;
		class?: string;
	}

	let {
		title,
		subtitle,
		gradient = false,
		centered = false,
		actions,
		class: className = ''
	}: Props = $props();
</script>

<div class="mb-8 {centered ? 'text-center' : 'flex items-center justify-between'} {className}">
	<div>
		<h1 class="text-3xl sm:text-4xl font-bold text-gray-800" style="font-family: 'Baloo 2', sans-serif;">
			{#if gradient}
				<GradientText>{title}</GradientText>
			{:else}
				{title}
			{/if}
		</h1>
		{#if subtitle}
			<p class="mt-2 text-lg text-gray-600">{subtitle}</p>
		{/if}
	</div>
	{#if actions && !centered}
		<div class="flex items-center gap-3">
			{@render actions()}
		</div>
	{/if}
</div>
