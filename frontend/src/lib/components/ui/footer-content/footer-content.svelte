<script lang="ts">
	import { i18n, useTranslation, type Locale } from '$lib/i18n/index.svelte';
	import { SelectNative } from '$lib/components/ui/select-native';
	import { Globe } from 'lucide-svelte';

	const t = useTranslation();
</script>

<div class="flex flex-col sm:flex-row items-center justify-between gap-4">
	<!-- Logo & Copyright -->
	<div class="flex items-center gap-2 text-sm" style="color: var(--color-subtle);">
		<img src="/mascot.png" alt="Loafy" class="h-6 w-6 object-contain opacity-70" />
		<span>&copy; {new Date().getFullYear()} Loafy Club</span>
	</div>

	<!-- Tagline & Language Selector -->
	<div class="flex items-center gap-6">
		<p class="text-sm hidden sm:block" style="color: var(--color-subtle);">
			{t('home.footer.tagline')}
		</p>

		<!-- Language Selector -->
		<div class="flex items-center gap-2">
			<Globe class="h-4 w-4" style="color: var(--color-subtle);" />
			<SelectNative
				value={i18n.locale}
				onchange={(e) => i18n.setLocale(e.currentTarget.value as Locale)}
				variant="compact"
				style="background-color: var(--color-card); border-color: var(--color-border); color: var(--color-foreground);"
			>
				<option value="en">{t('languages.en')}</option>
				<option value="vi">{t('languages.vi')}</option>
			</SelectNative>
		</div>
	</div>
</div>
