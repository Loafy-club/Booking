<script lang="ts">
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { Card } from '$lib/components/ui/card';
</script>

<!-- Back button -->
<div class="mb-6">
	<Skeleton class="h-9 w-36" />
</div>

<div class="grid gap-6 lg:grid-cols-3">
	<!-- Booking details -->
	<div class="lg:col-span-2 space-y-6">
		<Card variant="glass" class="[&>div:first-child]:hidden [&>div:last-child]:shadow-none">
			<!-- Header with code and badges -->
			<div class="flex items-center gap-3 flex-wrap mb-6">
				<Skeleton class="h-8 w-48" />
				<Skeleton class="h-5 w-20 rounded-md" />
			</div>

			<!-- Details grid -->
			<div class="grid gap-4 sm:grid-cols-2">
				{#each Array(6) as _}
					<div>
						<Skeleton class="h-3 w-24 mb-1" />
						<Skeleton class="h-5 w-32" />
					</div>
				{/each}
			</div>
		</Card>

		<!-- Session info card -->
		<Card variant="glass" class="[&>div:first-child]:hidden [&>div:last-child]:shadow-none">
			<Skeleton class="h-6 w-32 mb-4" />
			<div class="space-y-3">
				{#each Array(3) as _}
					<div class="flex items-center gap-2">
						<Skeleton class="h-4 w-4 rounded" />
						<Skeleton class="h-4 w-48" />
					</div>
				{/each}
			</div>
		</Card>
	</div>

	<!-- Actions sidebar -->
	<div class="lg:col-span-1">
		<Card variant="glass" class="[&>div:first-child]:hidden [&>div:last-child]:shadow-none">
			<Skeleton class="h-6 w-24 mb-4" />
			<div class="space-y-3">
				<Skeleton class="h-10 w-full rounded-md" />
				<Skeleton class="h-10 w-full rounded-md" />
			</div>
		</Card>
	</div>
</div>
