<script lang="ts">
	import type { Snippet } from 'svelte';
	import { GlassCard } from '$lib/components/ui/glass-card';
	import { Button } from '$lib/components/ui/button';

	interface Props {
		title: string;
		description?: string;
		icon?: Snippet;
		actionText?: string;
		onAction?: () => void;
		class?: string;
	}

	let {
		title,
		description,
		icon,
		actionText,
		onAction,
		class: className = ''
	}: Props = $props();
</script>

<div class="mx-auto max-w-md {className}">
	<GlassCard variant="default">
		<div class="text-center py-4">
			{#if icon}
				<div class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-gray-100">
					{@render icon()}
				</div>
			{:else}
				<div class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-gray-100">
					<svg class="h-8 w-8 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4" />
					</svg>
				</div>
			{/if}
			<h3 class="text-xl font-semibold text-gray-800" style="font-family: 'Baloo 2', sans-serif;">{title}</h3>
			{#if description}
				<p class="mt-2 text-gray-600">{description}</p>
			{/if}
			{#if actionText && onAction}
				<Button class="mt-6" onclick={onAction}>{actionText}</Button>
			{/if}
		</div>
	</GlassCard>
</div>
