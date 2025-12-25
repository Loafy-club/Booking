<script lang="ts">
	import { useTranslation } from '$lib/i18n/index.svelte';
	import { AppLayout } from '$lib/components/layouts';
	import { Card } from '$lib/components/ui/card';
	import { SectionHeader } from '$lib/components/ui/section-header';
	import { AnimatedContainer } from '$lib/components/ui/animated-container';
	import { Mail, Clock, Users, Facebook, Instagram } from 'lucide-svelte';
	import { ZaloIcon } from '$lib/components/icons';

	const t = useTranslation();

	const socials = [
		{
			name: 'facebook',
			url: 'https://facebook.com/loafy.club',
			icon: Facebook,
			bgColor: 'bg-[#1877F2]',
			hoverColor: 'hover:bg-[#166FE5]'
		},
		{
			name: 'instagram',
			url: 'https://instagram.com/loafy.club',
			icon: Instagram,
			bgColor: 'bg-gradient-to-br from-[#F58529] via-[#DD2A7B] to-[#8134AF]',
			hoverColor: 'hover:opacity-90'
		},
		{
			name: 'zalo',
			url: '#', // TODO: Add Zalo group link
			icon: ZaloIcon,
			bgColor: 'bg-[#0068FF]',
			hoverColor: 'hover:bg-[#0055CC]'
		}
	];
</script>

<svelte:head>
	<title>{t('contact.title')} - Loafy Club</title>
</svelte:head>

<AppLayout>
	<!-- Header with Mascot -->
	<AnimatedContainer animation="fade-up">
		<div class="flex flex-col items-center mb-8">
			<img
				src="/mascot.png"
				alt="Loafy Mascot"
				class="h-28 w-28 object-contain animate-float-slow mb-4"
			/>
			<SectionHeader
				title={t('contact.title')}
				subtitle={t('contact.description')}
				gradient
				centered
			/>
		</div>
	</AnimatedContainer>

	<!-- Cards Grid -->
	<AnimatedContainer animation="fade-up" delay={100}>
		<div class="flex flex-col sm:flex-row gap-6 max-w-2xl mx-auto items-stretch">
			<!-- Email Card -->
			<div class="flex-1">
				<Card variant="glassOrange" class="h-full text-center hover:-translate-y-2 transition-transform duration-300">
					<div class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-orange-500/20">
						<Mail class="h-8 w-8 text-orange-500" />
					</div>
					<h3 class="font-display text-lg font-semibold mb-2">{t('contact.emailLabel')}</h3>
					<a
						href="mailto:support@loafy.club"
						class="text-primary hover:underline font-medium"
					>
						support@loafy.club
					</a>
				</Card>
			</div>

			<!-- Socials Card -->
			<div class="flex-1">
				<Card variant="glassPink" class="h-full text-center hover:-translate-y-2 transition-transform duration-300">
					<div class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-pink-500/20">
						<Users class="h-8 w-8 text-pink-500" />
					</div>
					<h3 class="font-display text-lg font-semibold mb-4">{t('contact.socialsLabel')}</h3>
					<div class="flex justify-center gap-4">
						{#each socials as social}
							<a
								href={social.url}
								target="_blank"
								rel="noopener noreferrer"
								class="flex items-center justify-center h-12 w-12 rounded-full {social.bgColor} {social.hoverColor} transition-all duration-200 hover:scale-110 shadow-lg"
								title={t(`contact.${social.name}`)}
							>
								<social.icon class="h-6 w-6 text-white" />
							</a>
						{/each}
					</div>
				</Card>
			</div>
		</div>
	</AnimatedContainer>

	<!-- Response Time Note -->
	<AnimatedContainer animation="fade-up" delay={300}>
		<div class="flex items-center justify-center gap-2 mt-8 text-muted-foreground">
			<Clock class="h-4 w-4" />
			<p class="text-sm">{t('contact.responseTime')}</p>
		</div>
	</AnimatedContainer>
</AppLayout>
