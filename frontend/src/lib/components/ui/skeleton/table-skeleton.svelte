<script lang="ts">
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { Card } from '$lib/components/ui/card';

	type Props = {
		rows?: number;
		cols?: number;
	};

	let { rows = 5, cols = 6 }: Props = $props();
</script>

<Card variant="glass" class="overflow-hidden !p-0 [&>div:first-child]:hidden [&>div:last-child]:shadow-none">
	<div class="overflow-x-auto">
		<table class="w-full">
			<thead class="border-b border-border">
				<tr>
					{#each Array(cols) as _, i}
						<th class="h-12 px-4 text-left align-middle">
							<Skeleton class="h-4 w-20" />
						</th>
					{/each}
				</tr>
			</thead>
			<tbody>
				{#each Array(rows) as _}
					<tr class="border-b border-border last:border-0">
						{#each Array(cols) as _, i}
							<td class="p-4 align-middle">
								{#if i === 0}
									<Skeleton class="h-5 w-32 mb-1" />
									<Skeleton class="h-3 w-24" />
								{:else if i === cols - 1}
									<div class="flex justify-end">
										<Skeleton class="h-8 w-8 rounded-md" />
									</div>
								{:else}
									<Skeleton class="h-4 w-20" />
								{/if}
							</td>
						{/each}
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
</Card>
