<script lang="ts" module>
	import { type VariantProps, tv } from "tailwind-variants";

	export const glassCardVariants = tv({
		slots: {
			wrapper: "relative group",
			glow: "absolute inset-0 bg-gradient-to-br rounded-[2rem] blur-xl transition-all duration-500",
			content: "relative rounded-[2rem] shadow-lg transition-all duration-500 backdrop-blur-sm",
		},
		variants: {
			variant: {
				default: {
					glow: "from-orange-300/40 to-pink-300/40 opacity-40",
				},
				orange: {
					glow: "from-yellow-300 to-orange-400 opacity-40",
				},
				pink: {
					glow: "from-orange-300 to-pink-400 opacity-40",
				},
				yellow: {
					glow: "from-pink-300 to-rose-400 opacity-40",
				},
			},
			hover: {
				true: {
					glow: "group-hover:opacity-60 group-hover:scale-105",
					content: "hover:shadow-2xl hover:-translate-y-2",
				},
				false: {},
			},
			padding: {
				default: { content: "p-8" },
				sm: { content: "p-4" },
				md: { content: "p-6" },
				lg: { content: "p-8" },
				xl: { content: "p-10" },
				none: { content: "p-0" },
			},
		},
		defaultVariants: {
			variant: "default",
			hover: false,
			padding: "default",
		},
	});

	export type GlassCardVariant = VariantProps<typeof glassCardVariants>["variant"];
	export type GlassCardPadding = VariantProps<typeof glassCardVariants>["padding"];
</script>

<script lang="ts">
	import type { Snippet } from 'svelte';
	import { cn } from "$lib/utils";

	interface Props {
		children: Snippet;
		variant?: GlassCardVariant;
		hover?: boolean;
		padding?: GlassCardPadding;
		class?: string;
	}

	let { children, variant = 'default', hover = false, padding = 'default', class: className = '' }: Props = $props();

	const styles = $derived(glassCardVariants({ variant, hover, padding }));
</script>

<div class={cn(styles.wrapper(), className)}>
	<!-- Glow effect -->
	<div class={styles.glow()}></div>

	<!-- Card content -->
	<div class={styles.content()} style="background-color: var(--color-glass);">
		{@render children()}
	</div>
</div>
