<script lang="ts" module>
	import { type VariantProps, tv } from "tailwind-variants";

	export const pageBackgroundVariants = tv({
		base: "relative min-h-screen flex flex-col",
		variants: {
			variant: {
				default: "",
				hero: "",
				subtle: "",
			},
		},
		defaultVariants: {
			variant: "default",
		},
	});

	export type PageBackgroundVariant = VariantProps<typeof pageBackgroundVariants>["variant"];
</script>

<script lang="ts">
	import type { Snippet } from 'svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { cn } from "$lib/utils";

	interface Props {
		children: Snippet;
		variant?: PageBackgroundVariant;
		class?: string;
		noFooter?: boolean;
	}

	let { children, variant = 'default', class: className = '', noFooter = false }: Props = $props();
</script>

<div
	class={cn(pageBackgroundVariants({ variant }), "bg-gradient-to-br from-page-from via-page-via to-page-to", className)}
>
	<!-- Floating decorative blobs -->
	<div class="absolute inset-0 overflow-hidden pointer-events-none">
		<div class="absolute top-10 left-10 w-64 h-64 rounded-full blur-3xl animate-float-slow bg-blob-yellow"></div>
		<div class="absolute bottom-20 right-10 w-80 h-80 rounded-full blur-3xl animate-float-slower bg-blob-orange"></div>
		<div class="absolute top-1/3 right-1/4 w-48 h-48 rounded-full blur-3xl animate-float bg-blob-pink"></div>
		<div class="absolute bottom-1/3 left-1/4 w-56 h-56 rounded-full blur-3xl animate-float-slow bg-blob-rose"></div>
	</div>

	<div class="relative flex-1">
		{@render children()}
	</div>

	{#if !noFooter}
		<Footer />
	{/if}
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
