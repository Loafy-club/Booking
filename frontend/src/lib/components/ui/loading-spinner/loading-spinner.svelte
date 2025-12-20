<script lang="ts">
	interface Props {
		size?: 'sm' | 'md' | 'lg';
		text?: string;
		class?: string;
	}

	let { size = 'md', text, class: className = '' }: Props = $props();

	const sizes = {
		sm: 'h-6 w-6 border-2',
		md: 'h-12 w-12 border-4',
		lg: 'h-16 w-16 border-4'
	};
</script>

<div class="flex flex-col items-center justify-center py-12 {className}">
	<div class="animate-spin rounded-full border-primary border-t-transparent {sizes[size]}"></div>
	{#if text}
		<p class="mt-4 text-muted-foreground">{text}</p>
	{/if}
</div>
