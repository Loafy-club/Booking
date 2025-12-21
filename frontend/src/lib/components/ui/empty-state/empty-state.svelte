<script lang="ts" module>
	import { type VariantProps, tv } from "tailwind-variants";

	export const emptyStateVariants = tv({
		slots: {
			wrapper: "mx-auto max-w-md",
			content: "text-center py-4",
			iconWrapper: "mx-auto mb-4 flex items-center justify-center rounded-full bg-gray-100",
			icon: "text-gray-400",
			title: "font-semibold text-gray-800 font-display",
			description: "text-gray-600",
			action: "",
		},
		variants: {
			size: {
				default: {
					iconWrapper: "h-16 w-16",
					icon: "h-8 w-8",
					title: "text-xl",
					description: "mt-2",
					action: "mt-6",
				},
				sm: {
					iconWrapper: "h-12 w-12",
					icon: "h-6 w-6",
					title: "text-lg",
					description: "mt-1 text-sm",
					action: "mt-4",
				},
				lg: {
					iconWrapper: "h-20 w-20",
					icon: "h-10 w-10",
					title: "text-2xl",
					description: "mt-3 text-lg",
					action: "mt-8",
				},
			},
		},
		defaultVariants: {
			size: "default",
		},
	});

	export type EmptyStateSize = VariantProps<typeof emptyStateVariants>["size"];
</script>

<script lang="ts">
	import type { Snippet } from 'svelte';
	import { GlassCard } from '$lib/components/ui/glass-card';
	import { Button } from '$lib/components/ui/button';
	import { Inbox } from 'lucide-svelte';
	import { cn } from "$lib/utils";

	interface Props {
		title: string;
		description?: string;
		icon?: Snippet;
		actionText?: string;
		onAction?: () => void;
		size?: EmptyStateSize;
		class?: string;
	}

	let {
		title,
		description,
		icon,
		actionText,
		onAction,
		size = 'default',
		class: className = ''
	}: Props = $props();

	const styles = $derived(emptyStateVariants({ size }));
</script>

<div class={cn(styles.wrapper(), className)}>
	<GlassCard variant="default">
		<div class={styles.content()}>
			{#if icon}
				<div class={styles.iconWrapper()}>
					{@render icon()}
				</div>
			{:else}
				<div class={styles.iconWrapper()}>
					<Inbox class={styles.icon()} />
				</div>
			{/if}
			<h3 class={styles.title()}>{title}</h3>
			{#if description}
				<p class={styles.description()}>{description}</p>
			{/if}
			{#if actionText && onAction}
				<Button class={styles.action()} onclick={onAction}>{actionText}</Button>
			{/if}
		</div>
	</GlassCard>
</div>
