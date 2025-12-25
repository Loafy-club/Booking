<script lang="ts">
	import { i18n, useTranslation, type Locale } from '$lib/i18n/index.svelte';
	import * as Select from '$lib/components/ui/select';

	type Props = {
		size?: 'sm' | 'default';
		class?: string;
	};

	let { size = 'default', class: className = '' }: Props = $props();

	const t = useTranslation();

	const languages: { value: Locale; flag: string; labelKey: string }[] = [
		{ value: 'en', flag: '🇬🇧', labelKey: 'languages.en' },
		{ value: 'vi', flag: '🇻🇳', labelKey: 'languages.vi' }
	];
</script>

<Select.Root type="single" value={i18n.locale} onValueChange={(value) => value && i18n.setLocale(value as Locale)}>
	<Select.Trigger {size} class={className}>
		<Select.Value>
			{#each languages as lang}
				{#if i18n.locale === lang.value}
					<span class="flex items-center gap-2">{lang.flag} {t(lang.labelKey)}</span>
				{/if}
			{/each}
		</Select.Value>
	</Select.Trigger>
	<Select.Content>
		{#each languages as lang}
			<Select.Item value={lang.value}>
				<span class="flex items-center gap-2">{lang.flag} {t(lang.labelKey)}</span>
			</Select.Item>
		{/each}
	</Select.Content>
</Select.Root>
