<script lang="ts">
	import { Ticket } from 'lucide-svelte';

	interface Props {
		count: number;
		variant?: 'default' | 'small' | 'inline';
		showLabel?: boolean;
	}

	let { count, variant = 'default', showLabel = false }: Props = $props();
</script>

{#if variant === 'inline'}
	<span class="inline-flex items-center gap-1 text-orange-600 dark:text-orange-400">
		<Ticket class="size-4" />
		<span class="font-medium">{count}</span>
	</span>
{:else if variant === 'small'}
	<div class="inline-flex items-center gap-1 rounded-full bg-orange-100 dark:bg-orange-900/30 px-2 py-0.5 text-xs font-medium text-orange-700 dark:text-orange-300">
		<Ticket class="size-3" />
		<span>{count}</span>
	</div>
{:else}
	<div class="flex items-center gap-2 rounded-lg bg-gradient-to-r from-orange-50 to-amber-50 dark:from-orange-900/20 dark:to-amber-900/20 px-3 py-2 border border-orange-200 dark:border-orange-800">
		<div class="flex items-center justify-center size-8 rounded-full bg-orange-100 dark:bg-orange-900/50">
			<Ticket class="size-4 text-orange-600 dark:text-orange-400" />
		</div>
		<div class="flex flex-col">
			<span class="text-lg font-bold text-orange-700 dark:text-orange-300">{count}</span>
			{#if showLabel}
				<span class="text-xs text-orange-600/70 dark:text-orange-400/70">tickets</span>
			{/if}
		</div>
	</div>
{/if}
