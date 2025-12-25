<script lang="ts">
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { Card } from '$lib/components/ui/card';
</script>

<!-- Back button -->
<div class="mb-6">
	<Skeleton class="h-9 w-36" />
</div>

<div class="grid gap-6 lg:grid-cols-3">
	<!-- Session details -->
	<div class="lg:col-span-2">
		<Card variant="glass" class="[&>div:first-child]:hidden [&>div:last-child]:shadow-none">
			<!-- Title -->
			<Skeleton class="h-9 w-3/4 mb-4" />

			<!-- Description -->
			<Skeleton class="h-4 w-full mb-2" />
			<Skeleton class="h-4 w-2/3 mb-6" />

			<!-- Details -->
			<div class="space-y-4">
				{#each Array(4) as _}
					<div class="flex items-start">
						<Skeleton class="mr-3 h-6 w-6 rounded" />
						<div class="flex-1">
							<Skeleton class="h-5 w-24 mb-1" />
							<Skeleton class="h-4 w-40" />
						</div>
					</div>
				{/each}
			</div>
		</Card>
	</div>

	<!-- Booking form -->
	<div class="lg:col-span-1">
		<Card variant="glass" class="[&>div:first-child]:hidden [&>div:last-child]:shadow-none">
			<Skeleton class="h-7 w-40 mb-6" />

			<div class="space-y-4">
				<!-- Guest select -->
				<div>
					<Skeleton class="h-4 w-32 mb-2" />
					<Skeleton class="h-10 w-full rounded-md" />
				</div>

				<!-- Payment method -->
				<div>
					<Skeleton class="h-4 w-32 mb-2" />
					<Skeleton class="h-14 w-full rounded-xl mb-2" />
					<Skeleton class="h-14 w-full rounded-xl" />
				</div>

				<!-- Total -->
				<div class="pt-4 border-t border-border">
					<div class="flex justify-between items-center">
						<Skeleton class="h-5 w-16" />
						<Skeleton class="h-8 w-28" />
					</div>
				</div>

				<!-- Book button -->
				<Skeleton class="h-12 w-full rounded-md" />
			</div>
		</Card>
	</div>
</div>
