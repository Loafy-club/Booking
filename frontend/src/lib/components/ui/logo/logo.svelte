<script lang="ts" module>
	import { type VariantProps, tv } from "tailwind-variants";

	export const logoVariants = tv({
		slots: {
			wrapper: "inline-flex items-center gap-3 group",
			image: "relative object-contain drop-shadow-lg group-hover:scale-110 transition-transform duration-300",
			text: "font-bold text-gradient-primary font-display",
			ring: "rounded-full border-4 border-dashed border-orange-200/60 animate-spin-very-slow",
		},
		variants: {
			size: {
				sm: { image: "h-8 w-8", text: "text-xl", ring: "w-12 h-12" },
				md: { image: "h-12 w-12", text: "text-2xl", ring: "w-16 h-16" },
				lg: { image: "h-16 w-16", text: "text-3xl", ring: "w-24 h-24" },
				xl: { image: "h-24 w-24", text: "text-4xl", ring: "w-32 h-32" },
			},
		},
		defaultVariants: {
			size: "md",
		},
	});

	export type LogoSize = VariantProps<typeof logoVariants>["size"];
</script>

<script lang="ts">
	import { cn } from "$lib/utils";

	interface Props {
		size?: LogoSize;
		showText?: boolean;
		showRing?: boolean;
		href?: string;
		class?: string;
	}

	let { size = 'md', showText = true, showRing = false, href = '/', class: className = '' }: Props = $props();

	const styles = $derived(logoVariants({ size }));
</script>

<a {href} class={cn(styles.wrapper(), className)}>
	<div class="relative">
		{#if showRing}
			<div class="absolute inset-0 flex items-center justify-center">
				<div class={styles.ring()}></div>
			</div>
		{/if}
		<img
			src="/mascot.png"
			alt="Loafy the Corgi"
			class={styles.image()}
		/>
	</div>
	{#if showText}
		<span class={styles.text()}>
			Loafy Club
		</span>
	{/if}
</a>

<style>
	@keyframes spin-very-slow {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}
	:global(.animate-spin-very-slow) {
		animation: spin-very-slow 30s linear infinite;
	}
</style>
