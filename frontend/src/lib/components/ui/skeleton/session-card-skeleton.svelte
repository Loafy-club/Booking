<script lang="ts">
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { Card } from '$lib/components/ui/card';

	type Props = {
		variant?: 'glassYellow' | 'glassOrange' | 'glassPink';
	};

	let { variant = 'glassOrange' }: Props = $props();
</script>

<Card {variant} class="[&>div:first-child]:hidden [&>div:last-child]:shadow-none">
	<!-- Title and badge -->
	<div class="flex items-start justify-between gap-2 mb-4">
		<Skeleton class="h-6 w-3/4" />
		<Skeleton class="h-5 w-16 rounded-md" />
	</div>

	<!-- Location -->
	<div class="flex items-center gap-2 mb-4">
		<Skeleton class="h-4 w-4 rounded" />
		<Skeleton class="h-4 w-32" />
	</div>

	<div class="space-y-3">
		<!-- Date & Time -->
		<div class="flex items-center gap-2">
			<Skeleton class="h-4 w-4 rounded" />
			<Skeleton class="h-4 w-24" />
			<Skeleton class="h-4 w-16" />
		</div>

		<!-- Courts & Players -->
		<div class="flex items-center gap-2">
			<Skeleton class="h-4 w-4 rounded" />
			<Skeleton class="h-4 w-20" />
			<Skeleton class="h-4 w-24" />
		</div>

		<!-- Availability -->
		<div class="flex items-center gap-2">
			<Skeleton class="h-4 w-4 rounded" />
			<Skeleton class="h-4 w-12" />
			<Skeleton class="h-4 w-28" />
		</div>

		<!-- Price -->
		<div class="pt-3 border-t border-border">
			<div class="flex items-center justify-between">
				<Skeleton class="h-4 w-24" />
				<Skeleton class="h-6 w-20" />
			</div>
		</div>
	</div>

	<!-- Button -->
	<Skeleton class="h-10 w-full mt-4 rounded-md" />
</Card>
