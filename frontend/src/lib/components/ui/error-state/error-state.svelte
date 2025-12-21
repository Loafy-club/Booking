<script lang="ts" module>
	import { type VariantProps, tv } from "tailwind-variants";

	export const errorStateVariants = tv({
		slots: {
			wrapper: "mx-auto max-w-md",
			content: "text-center",
			iconWrapper: "mx-auto mb-4 flex items-center justify-center rounded-full bg-red-100",
			icon: "text-red-600",
			title: "font-semibold text-gray-800 font-display",
			message: "text-gray-600",
			action: "",
		},
		variants: {
			size: {
				default: {
					iconWrapper: "h-12 w-12",
					icon: "h-6 w-6",
					title: "text-lg",
					message: "mt-2",
					action: "mt-6",
				},
				sm: {
					iconWrapper: "h-10 w-10",
					icon: "h-5 w-5",
					title: "text-base",
					message: "mt-1 text-sm",
					action: "mt-4",
				},
				lg: {
					iconWrapper: "h-16 w-16",
					icon: "h-8 w-8",
					title: "text-xl",
					message: "mt-3 text-lg",
					action: "mt-8",
				},
			},
		},
		defaultVariants: {
			size: "default",
		},
	});

	export type ErrorStateSize = VariantProps<typeof errorStateVariants>["size"];
</script>

<script lang="ts">
	import { GlassCard } from '$lib/components/ui/glass-card';
	import { Button } from '$lib/components/ui/button';
	import { AlertTriangle } from 'lucide-svelte';
	import { cn } from "$lib/utils";

	interface Props {
		title?: string;
		message: string;
		onRetry?: () => void;
		retryText?: string;
		size?: ErrorStateSize;
		class?: string;
	}

	let {
		title = 'Error',
		message,
		onRetry,
		retryText = 'Try Again',
		size = 'default',
		class: className = ''
	}: Props = $props();

	const styles = $derived(errorStateVariants({ size }));
</script>

<div class={cn(styles.wrapper(), className)}>
	<GlassCard variant="default">
		<div class={styles.content()}>
			<div class={styles.iconWrapper()}>
				<AlertTriangle class={styles.icon()} />
			</div>
			<h3 class={styles.title()}>{title}</h3>
			<p class={styles.message()}>{message}</p>
			{#if onRetry}
				<Button class={styles.action()} onclick={onRetry}>{retryText}</Button>
			{/if}
		</div>
	</GlassCard>
</div>
