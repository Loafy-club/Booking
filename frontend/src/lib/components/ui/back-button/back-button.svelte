<script lang="ts" module>
	import { type VariantProps, tv } from "tailwind-variants";

	export const backButtonVariants = tv({
		slots: {
			wrapper: "mb-6",
			button: "gap-2 text-gray-600 hover:text-gray-800",
			icon: "h-4 w-4",
		},
		variants: {
			size: {
				default: { wrapper: "mb-6", icon: "h-4 w-4" },
				sm: { wrapper: "mb-4", icon: "h-3 w-3" },
				lg: { wrapper: "mb-8", icon: "h-5 w-5" },
			},
		},
		defaultVariants: {
			size: "default",
		},
	});

	export type BackButtonSize = VariantProps<typeof backButtonVariants>["size"];
</script>

<script lang="ts">
	import { goto } from '$app/navigation';
	import { Button } from '$lib/components/ui/button';
	import { ArrowLeft } from 'lucide-svelte';
	import { cn } from "$lib/utils";

	interface Props {
		href: string;
		label?: string;
		size?: BackButtonSize;
		class?: string;
	}

	let { href, label = 'Back', size = 'default', class: className = '' }: Props = $props();

	const styles = $derived(backButtonVariants({ size }));
</script>

<div class={cn(styles.wrapper(), className)}>
	<Button variant="ghost" onclick={() => goto(href)} class={styles.button()}>
		<ArrowLeft class={styles.icon()} />
		{label}
	</Button>
</div>
