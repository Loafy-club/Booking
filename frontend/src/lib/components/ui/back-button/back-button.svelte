<script lang="ts">
	import { goto } from '$app/navigation';
	import { Button } from '$lib/components/ui/button';

	interface Props {
		href: string;
		label?: string;
		class?: string;
	}

	let { href, label = 'Back', class: className = '' }: Props = $props();
</script>

<div class="mb-6 {className}">
	<Button variant="ghost" onclick={() => goto(href)} class="gap-2 text-gray-600 hover:text-gray-800">
		<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
			<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
		</svg>
		{label}
	</Button>
</div>
