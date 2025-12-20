<script lang="ts">
	interface Props {
		status: string;
		variant?: 'booking' | 'session' | 'custom';
		customColors?: { bg: string; text: string };
		class?: string;
	}

	let { status, variant = 'booking', customColors, class: className = '' }: Props = $props();

	function getColors(): { bg: string; text: string } {
		if (customColors) return customColors;

		if (variant === 'booking') {
			switch (status.toLowerCase()) {
				case 'pending':
					return { bg: 'bg-yellow-100', text: 'text-yellow-800' };
				case 'confirmed':
					return { bg: 'bg-green-100', text: 'text-green-800' };
				case 'failed':
				case 'cancelled':
					return { bg: 'bg-red-100', text: 'text-red-800' };
				default:
					return { bg: 'bg-gray-100', text: 'text-gray-800' };
			}
		}

		if (variant === 'session') {
			switch (status.toLowerCase()) {
				case 'draft':
					return { bg: 'bg-gray-100', text: 'text-gray-800' };
				case 'published':
				case 'upcoming':
					return { bg: 'bg-green-100', text: 'text-green-800' };
				case 'in_progress':
					return { bg: 'bg-blue-100', text: 'text-blue-800' };
				case 'completed':
					return { bg: 'bg-purple-100', text: 'text-purple-800' };
				case 'cancelled':
					return { bg: 'bg-red-100', text: 'text-red-800' };
				case 'full':
					return { bg: 'bg-orange-100', text: 'text-orange-800' };
				default:
					return { bg: 'bg-gray-100', text: 'text-gray-800' };
			}
		}

		return { bg: 'bg-gray-100', text: 'text-gray-800' };
	}

	const colors = $derived(getColors());
</script>

<span class="inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium capitalize {colors.bg} {colors.text} {className}">
	{status.replace('_', ' ')}
</span>
