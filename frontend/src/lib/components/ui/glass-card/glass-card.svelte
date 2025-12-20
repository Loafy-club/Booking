<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
		variant?: 'default' | 'orange' | 'pink' | 'yellow';
		hover?: boolean;
		class?: string;
	}

	let { children, variant = 'default', hover = false, class: className = '' }: Props = $props();

	const glowColors = {
		default: 'from-orange-300/40 to-pink-300/40',
		orange: 'from-yellow-300 to-orange-400',
		pink: 'from-orange-300 to-pink-400',
		yellow: 'from-pink-300 to-rose-400'
	};
</script>

<div class="relative group {className}">
	<!-- Glow effect -->
	<div
		class="absolute inset-0 bg-gradient-to-br {glowColors[variant]} rounded-[2rem] blur-xl opacity-40 transition-all duration-500 {hover ? 'group-hover:opacity-60 group-hover:scale-105' : ''}"
	></div>

	<!-- Card content -->
	<div
		class="relative bg-white/80 backdrop-blur-sm rounded-[2rem] p-8 shadow-lg transition-all duration-500 {hover ? 'hover:shadow-2xl hover:-translate-y-2' : ''}"
	>
		{@render children()}
	</div>
</div>
