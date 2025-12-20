<script lang="ts">
	import { GlassCard } from '$lib/components/ui/glass-card';
	import { Button } from '$lib/components/ui/button';

	interface Props {
		title?: string;
		message: string;
		onRetry?: () => void;
		retryText?: string;
		class?: string;
	}

	let {
		title = 'Error',
		message,
		onRetry,
		retryText = 'Try Again',
		class: className = ''
	}: Props = $props();
</script>

<div class="mx-auto max-w-md {className}">
	<GlassCard variant="default">
		<div class="text-center">
			<div class="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full bg-red-100">
				<svg class="h-6 w-6 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
				</svg>
			</div>
			<h3 class="text-lg font-semibold text-gray-800" style="font-family: 'Baloo 2', sans-serif;">{title}</h3>
			<p class="mt-2 text-gray-600">{message}</p>
			{#if onRetry}
				<Button class="mt-6" onclick={onRetry}>{retryText}</Button>
			{/if}
		</div>
	</GlassCard>
</div>
