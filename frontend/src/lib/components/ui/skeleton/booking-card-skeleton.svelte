<script lang="ts">
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { Card } from '$lib/components/ui/card';
</script>

<Card variant="glass" class="[&>div:first-child]:hidden [&>div:last-child]:shadow-none">
	<div class="flex items-start justify-between">
		<div class="flex-1">
			<!-- Title and badges -->
			<div class="flex items-center gap-3 flex-wrap">
				<Skeleton class="h-7 w-48" />
				<Skeleton class="h-5 w-20 rounded-md" />
			</div>

			<!-- Details grid -->
			<div class="mt-4 grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
				{#each Array(4) as _}
					<div>
						<Skeleton class="h-3 w-20 mb-1" />
						<Skeleton class="h-5 w-28" />
					</div>
				{/each}
			</div>
		</div>
	</div>

	<!-- Action buttons -->
	<div class="mt-6 pt-4 border-t border-border flex items-center gap-3">
		<Skeleton class="h-8 w-28 rounded-md" />
		<Skeleton class="h-8 w-36 rounded-md" />
	</div>
</Card>
