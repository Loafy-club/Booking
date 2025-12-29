<script lang="ts">
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { Card } from '$lib/components/ui/card';
</script>

<!-- Back button -->
<div class="mb-6">
	<Skeleton class="h-9 w-40" />
</div>

<Card variant="glass">
	<!-- Header with code, date and badge -->
	<div class="mb-6 flex items-center justify-between flex-wrap gap-4">
		<div>
			<Skeleton class="h-9 w-64 mb-2" />
			<Skeleton class="h-4 w-40" />
		</div>
		<Skeleton class="h-6 w-28 rounded-full" />
	</div>

	<div class="space-y-6">
		<!-- Info cards grid (2x2) -->
		<div class="grid gap-6 md:grid-cols-2">
			{#each Array(4) as _}
				<Card variant="info">
					<Skeleton class="h-4 w-28 mb-2" />
					<Skeleton class="h-6 w-36" />
				</Card>
			{/each}
		</div>

		<!-- Price breakdown -->
		<div class="border-t border-border pt-6">
			<Skeleton class="h-4 w-32 mb-3" />
			<Card variant="info" class="space-y-2">
				<div class="flex justify-between">
					<Skeleton class="h-4 w-20" />
					<Skeleton class="h-4 w-24" />
				</div>
			</Card>
		</div>

		<!-- Alert placeholder -->
		<Skeleton class="h-14 w-full rounded-lg" />

		<!-- Action buttons -->
		<div class="mt-8 flex gap-4 flex-wrap border-t border-border pt-6">
			<Skeleton class="h-11 w-40 rounded-md" />
			<Skeleton class="h-11 w-36 rounded-md" />
			<Skeleton class="h-11 w-36 rounded-md" />
		</div>
	</div>
</Card>
