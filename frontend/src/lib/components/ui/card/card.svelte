<script lang="ts">
	import { cn } from "$lib/utils";
	import type { HTMLAttributes } from "svelte/elements";

	type Props = HTMLAttributes<HTMLDivElement>;

	let { class: className, children, ...restProps }: Props = $props();
</script>

<div
	class={cn(
		"rounded-xl border bg-card text-card-foreground shadow",
		className
	)}
	{...restProps}
>
	{@render children?.()}
</div>
