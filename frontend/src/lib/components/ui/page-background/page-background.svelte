<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
		variant?: 'default' | 'hero' | 'subtle';
		class?: string;
	}

	let { children, variant = 'default', class: className = '' }: Props = $props();

	const gradients = {
		default: 'from-sky-100 via-orange-50 to-rose-100',
		hero: 'from-sky-100 via-orange-50 to-amber-50',
		subtle: 'from-amber-50 via-orange-50/50 to-rose-50/50'
	};
</script>

<div class="relative min-h-screen overflow-hidden bg-gradient-to-br {gradients[variant]} {className}">
	<!-- Floating decorative blobs -->
	<div class="absolute top-10 left-10 w-64 h-64 bg-yellow-200/40 rounded-full blur-3xl animate-float-slow pointer-events-none"></div>
	<div class="absolute bottom-20 right-10 w-80 h-80 bg-orange-200/40 rounded-full blur-3xl animate-float-slower pointer-events-none"></div>
	<div class="absolute top-1/3 right-1/4 w-48 h-48 bg-pink-200/40 rounded-full blur-3xl animate-float pointer-events-none"></div>
	<div class="absolute bottom-1/3 left-1/4 w-56 h-56 bg-rose-200/30 rounded-full blur-3xl animate-float-slow pointer-events-none"></div>

	<div class="relative">
		{@render children()}
	</div>
</div>

<style>
	@keyframes float {
		0%, 100% { transform: translateY(0); }
		50% { transform: translateY(-15px); }
	}
	@keyframes float-slow {
		0%, 100% { transform: translateY(0) translateX(0); }
		50% { transform: translateY(-20px) translateX(10px); }
	}
	@keyframes float-slower {
		0%, 100% { transform: translateY(0) translateX(0); }
		50% { transform: translateY(-25px) translateX(-15px); }
	}

	:global(.animate-float) {
		animation: float 4s ease-in-out infinite;
	}
	:global(.animate-float-slow) {
		animation: float-slow 8s ease-in-out infinite;
	}
	:global(.animate-float-slower) {
		animation: float-slower 12s ease-in-out infinite;
	}
</style>
