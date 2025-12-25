<script lang="ts">
	import { useTranslation } from '$lib/i18n/index.svelte';
	import { LanguageSelector } from '$lib/components/ui/language-selector';
	import { Separator } from '$lib/components/ui/separator';

	const t = useTranslation();
	const currentYear = new Date().getFullYear();

	const socialLinks = [
		{
			key: 'facebook',
			href: 'https://www.facebook.com/groups/loafyclub',
			icon: 'M22 12c0-5.523-4.477-10-10-10S2 6.477 2 12c0 4.991 3.657 9.128 8.438 9.878v-6.987h-2.54V12h2.54V9.797c0-2.506 1.492-3.89 3.777-3.89 1.094 0 2.238.195 2.238.195v2.46h-1.26c-1.243 0-1.63.771-1.63 1.562V12h2.773l-.443 2.89h-2.33v6.988C18.343 21.128 22 16.991 22 12z'
		},
		{
			key: 'instagram',
			href: 'https://instagram.com/loafy.club',
			icon: 'M12 2.163c3.204 0 3.584.012 4.85.07 3.252.148 4.771 1.691 4.919 4.919.058 1.265.069 1.645.069 4.849 0 3.205-.012 3.584-.069 4.849-.149 3.225-1.664 4.771-4.919 4.919-1.266.058-1.644.07-4.85.07-3.204 0-3.584-.012-4.849-.07-3.26-.149-4.771-1.699-4.919-4.92-.058-1.265-.07-1.644-.07-4.849 0-3.204.013-3.583.07-4.849.149-3.227 1.664-4.771 4.919-4.919 1.266-.057 1.645-.069 4.849-.069zm0-2.163c-3.259 0-3.667.014-4.947.072-4.358.2-6.78 2.618-6.98 6.98-.059 1.281-.073 1.689-.073 4.948 0 3.259.014 3.668.072 4.948.2 4.358 2.618 6.78 6.98 6.98 1.281.058 1.689.072 4.948.072 3.259 0 3.668-.014 4.948-.072 4.354-.2 6.782-2.618 6.979-6.98.059-1.28.073-1.689.073-4.948 0-3.259-.014-3.667-.072-4.947-.196-4.354-2.617-6.78-6.979-6.98-1.281-.059-1.69-.073-4.949-.073zm0 5.838c-3.403 0-6.162 2.759-6.162 6.162s2.759 6.163 6.162 6.163 6.162-2.759 6.162-6.163c0-3.403-2.759-6.162-6.162-6.162zm0 10.162c-2.209 0-4-1.79-4-4 0-2.209 1.791-4 4-4s4 1.791 4 4c0 2.21-1.791 4-4 4zm6.406-11.845c-.796 0-1.441.645-1.441 1.44s.645 1.44 1.441 1.44c.795 0 1.439-.645 1.439-1.44s-.644-1.44-1.439-1.44z'
		},
		{
			key: 'zalo',
			href: 'https://zalo.me/g/loafyclub',
			icon: 'M12.49 10.272v-.45h1.347v6.322h-.77l-.144-.432a2.47 2.47 0 0 1-3.424-.288c-.996-1.14-.852-2.98.325-3.937a2.494 2.494 0 0 1 2.666-.295v-.92zm.015 2.71a1.323 1.323 0 1 0-2.646 0 1.323 1.323 0 0 0 2.646 0zM7.521 10.272h1.32v5.873H7.52v-5.873zm.666-2.03a.765.765 0 1 1 0 1.53.765.765 0 0 1 0-1.53zM17.235 16.145h-1.347v-3.28c0-.376-.129-.752-.568-.752-.44 0-.647.338-.647.714v3.318h-1.348v-5.873h.76l.145.417a2.07 2.07 0 0 1 1.39-.555c1.23 0 1.615.924 1.615 1.94v4.07zM12 2C6.477 2 2 6.477 2 12s4.477 10 10 10 10-4.477 10-10S17.523 2 12 2z'
		}
	];
</script>

<div class="space-y-8">
	<!-- Main Footer Content -->
	<div class="grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-4">
		<!-- Brand Section -->
		<div class="sm:col-span-2 lg:col-span-1">
			<a href="/" class="flex items-center gap-2 mb-3">
				<img src="/mascot.png" alt="Loafy" class="h-8 w-8 object-contain" />
				<span class="font-semibold text-foreground">Loafy Club</span>
			</a>
			<p class="text-sm text-muted-foreground leading-relaxed">
				{t('footer.description')}
			</p>
		</div>

		<!-- Quick Links -->
		<div>
			<h3 class="font-medium text-foreground mb-3">{t('footer.quickLinks')}</h3>
			<ul class="space-y-2">
				<li>
					<a
						href="/sessions"
						class="text-sm text-muted-foreground hover:text-foreground transition-colors"
					>
						{t('footer.sessions')}
					</a>
				</li>
				<li>
					<a
						href="/bookings"
						class="text-sm text-muted-foreground hover:text-foreground transition-colors"
					>
						{t('footer.myBookings')}
					</a>
				</li>
				<li>
					<a
						href="/account"
						class="text-sm text-muted-foreground hover:text-foreground transition-colors"
					>
						{t('footer.account')}
					</a>
				</li>
			</ul>
		</div>

		<!-- Connect -->
		<div>
			<h3 class="font-medium text-foreground mb-3">{t('footer.connect')}</h3>
			<ul class="space-y-2">
				<li>
					<a
						href="/contact"
						class="text-sm text-muted-foreground hover:text-foreground transition-colors"
					>
						{t('footer.contact')}
					</a>
				</li>
				{#each socialLinks as link}
					<li>
						<a
							href={link.href}
							target="_blank"
							rel="noopener noreferrer"
							class="inline-flex items-center gap-2 text-sm text-muted-foreground hover:text-foreground transition-colors"
						>
							<svg class="h-4 w-4" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
								<path d={link.icon} />
							</svg>
							{t(`footer.${link.key}`)}
						</a>
					</li>
				{/each}
			</ul>
		</div>

		<!-- Legal & Settings -->
		<div>
			<h3 class="font-medium text-foreground mb-3">{t('footer.legal')}</h3>
			<ul class="space-y-2">
				<li>
					<a
						href="/terms"
						class="text-sm text-muted-foreground hover:text-foreground transition-colors"
					>
						{t('footer.terms')}
					</a>
				</li>
				<li>
					<a
						href="/privacy"
						class="text-sm text-muted-foreground hover:text-foreground transition-colors"
					>
						{t('footer.privacy')}
					</a>
				</li>
			</ul>
		</div>
	</div>

	<Separator />

	<!-- Bottom Bar -->
	<div class="flex flex-col sm:flex-row items-center justify-between gap-4">
		<p class="text-sm text-muted-foreground">
			{t('footer.copyright', { year: currentYear.toString() })}
		</p>
		<LanguageSelector size="sm" class="h-8" />
	</div>
</div>
