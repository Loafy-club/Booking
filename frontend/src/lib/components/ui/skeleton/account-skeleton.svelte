<script lang="ts">
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { Card } from '$lib/components/ui/card';
</script>

<!-- Tabs -->
<div class="mb-8 flex justify-center">
	<Skeleton class="h-10 w-80 rounded-lg" />
</div>

<Card variant="glass" class="max-w-2xl mx-auto [&>div:first-child]:hidden [&>div:last-child]:shadow-none">
	<!-- Avatar and title -->
	<div class="flex items-center gap-4 mb-6">
		<Skeleton class="h-16 w-16 rounded-full" />
		<div>
			<Skeleton class="h-6 w-40 mb-2" />
			<Skeleton class="h-4 w-56" />
		</div>
	</div>

	<!-- Form fields -->
	<div class="space-y-6">
		{#each Array(4) as _}
			<div>
				<Skeleton class="h-4 w-24 mb-2" />
				<Skeleton class="h-10 w-full rounded-md" />
			</div>
		{/each}

		<!-- Save button -->
		<Skeleton class="h-10 w-32 rounded-md" />
	</div>
</Card>
