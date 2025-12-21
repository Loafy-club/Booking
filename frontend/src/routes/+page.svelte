<script lang="ts">
	import { onMount } from 'svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import { goto } from '$app/navigation';
	import Navigation from '$lib/components/Navigation.svelte';
	import { Button } from '$lib/components/ui/button';
	import { useTranslation } from '$lib/i18n/index.svelte';
	import { FooterContent } from '$lib/components/ui/footer-content';
	import { Calendar, Zap, Users } from 'lucide-svelte';

	const t = useTranslation();

	let scrollY = $state(0);
	let loaded = $state(false);
	let innerHeight = $state(800);

	onMount(() => {
		innerHeight = window.innerHeight;

		setTimeout(() => {
			loaded = true;
		}, 100);

		const observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						entry.target.classList.add('visible');
					}
				});
			},
			{ threshold: 0.1, rootMargin: '0px 0px -50px 0px' }
		);

		document.querySelectorAll('.scroll-reveal').forEach((el) => {
			observer.observe(el);
		});

		return () => observer.disconnect();
	});

	// Smooth opacity for parallax fade-out
	$effect(() => {
		if (typeof document !== 'undefined') {
			const heroOpacity = Math.max(0, 1 - scrollY / (innerHeight * 0.8));
			document.documentElement.style.setProperty('--hero-opacity', heroOpacity.toString());
		}
	});
</script>

<svelte:window bind:scrollY bind:innerHeight />

<svelte:head>
	<title>Loafy Club - Pickleball in Hanoi</title>
</svelte:head>

<div class="relative bg-gradient-to-b from-sky-100 via-orange-50 to-amber-50">
	<Navigation hidden={scrollY < 400} noSpacer />

	<!-- ===== PARALLAX HERO ===== -->
	<!-- Extra height (calc(100vh + 115px)) pushes wave below viewport initially -->
	<section class="relative h-[calc(100vh+90px)] min-h-[800px] overflow-hidden">
		<!-- Layer 1: Sky (slowest) -->
		<div class="absolute" style="transform: translateY({scrollY * 0.1}px)">
			<img
				src="/parallax-layers/sky.png"
				alt=""
				class="w-full h-full object-cover object-center opacity-0 transition-opacity duration-1000 {loaded ? 'opacity-100' : ''}"
			/>
		</div>

		<!-- Layer 2: Skyline -->
		<div class="absolute" style="transform: translateY({scrollY * 0.2}px)">
			<img
				src="/parallax-layers/skyline.png"
				alt=""
				class="object-contain mix-blend-multiply opacity-0 transition-opacity duration-1000 delay-200 {loaded ? 'opacity-100' : ''}"
			/>
		</div>

		<!-- Layer 3: Court -->
		<div class="absolute" style="transform: translateY({scrollY * 0.35}px)">
			<img
				src="/parallax-layers/court.png"
				alt=""
				class="w-full h-full object-cover object-top opacity-0 transition-opacity duration-1000 delay-300 {loaded ? 'opacity-100' : ''}"
			/>
		</div>

		<!-- Player 1 + Bubble 1 -->
		<div class="absolute z-[12] h-[90%]" style="transform: translateY({scrollY * 0.5}px)">
			<img
				src="/parallax-layers/player-1.png"
				alt="Pickleball player"
				class="h-full w-auto object-contain opacity-0 transition-all duration-700 delay-500 {loaded ? 'opacity-100 translate-x-0' : '-translate-x-10'}"
			/>
			<!-- Bubble 1 -->
			<div
				class="absolute left-[3%] top-[28%] z-20 opacity-0 {loaded ? 'animate-bubble-in' : ''}"
			>
				<div class="relative bg-white rounded-2xl px-5 py-3 shadow-lg animate-bubble">
					<p class="text-base text-gray-800 font-semibold whitespace-nowrap font-display">
						{t('home.bubbles.allLevels')}
					</p>
					<div class="absolute -bottom-2 right-4">
						<div class="w-0 h-0 border-l-8 border-l-transparent border-r-8 border-r-transparent border-t-10 border-t-white"></div>
					</div>
				</div>
			</div>
		</div>

		<!-- Player 2 + Bubble 2 -->
		<div class="absolute z-[11] h-[90%]" style="transform: translateY({scrollY * 0.45}px)">
			<img
				src="/parallax-layers/player-2.png"
				alt="Pickleball player"
				class="h-full w-auto object-contain opacity-0 transition-all duration-700 delay-600 {loaded ? 'opacity-100 translate-x-0' : '-translate-x-10'}"
			/>
			<!-- Bubble 2 -->
			<div
				class="absolute left-[28%] top-[51%] z-20 opacity-0 {loaded ? 'animate-bubble-in-delay-1' : ''}"
			>
				<div class="relative bg-white rounded-2xl px-5 py-3 shadow-lg animate-bubble-delay-1">
					<p class="text-base text-gray-800 font-semibold whitespace-nowrap font-display">
						{t('home.bubbles.indoor')}
					</p>
					<!-- Tail pointing left toward player 2 -->
					<div class="absolute -left-2 top-1/2 -translate-y-1/2">
						<div class="w-0 h-0 border-t-8 border-t-transparent border-b-8 border-b-transparent border-r-10 border-r-white"></div>
					</div>
				</div>
			</div>
		</div>

		<!-- Player 3 + Bubble 3 -->
		<div class="absolute z-[12] h-[90%]" style="transform: translateY({scrollY * 0.45}px)">
			<img
				src="/parallax-layers/player-3.png"
				alt="Pickleball player"
				class="h-full w-auto object-contain opacity-0 transition-all duration-700 delay-600 {loaded ? 'opacity-100 translate-x-0' : 'translate-x-10'}"
			/>
			<!-- Bubble 3 -->
			<div
				class="absolute right-[7%] top-[32%] z-20 opacity-0 {loaded ? 'animate-bubble-in-delay-2' : ''}"
			>
				<div class="relative bg-white rounded-2xl px-5 py-3 shadow-lg animate-bubble-delay-2">
					<p class="text-base text-gray-800 font-semibold whitespace-nowrap font-display">
						{t('home.bubbles.solo')}
					</p>
					<div class="absolute -bottom-2 left-1/2 -translate-x-1/2">
						<div class="w-0 h-0 border-l-8 border-l-transparent border-r-8 border-r-transparent border-t-10 border-t-white"></div>
					</div>
				</div>
			</div>
		</div>

		<!-- Player 4 + Bubble 4 -->
		<div class="absolute z-[11] h-[90%]" style="transform: translateY({scrollY * 0.5}px)">
			<img
				src="/parallax-layers/player-4.png"
				alt="Pickleball player"
				class="h-full w-auto object-contain opacity-0 transition-all duration-700 delay-500 {loaded ? 'opacity-100 translate-x-0' : 'translate-x-10'}"
			/>
			<!-- Bubble 4 -->
			<div
				class="absolute right-[28%] top-[49%] z-20 opacity-0 {loaded ? 'animate-bubble-in-delay-3' : ''}"
			>
				<div class="relative bg-white rounded-2xl px-5 py-3 shadow-lg animate-bubble-delay-3">
					<p class="text-base text-gray-800 font-semibold whitespace-nowrap font-display">
						{t('home.bubbles.international')}
					</p>
					<!-- Tail pointing right toward player 4 -->
					<div class="absolute -right-2 top-1/2 -translate-y-1/2">
						<div class="w-0 h-0 border-t-8 border-t-transparent border-b-8 border-b-transparent border-l-10 border-l-white"></div>
					</div>
				</div>
			</div>
		</div>

		<!-- Hero Text -->
		<div class="absolute inset-x-0 top-[15%] flex justify-center pointer-events-none z-20"
			style="transform: translateY({scrollY * 0.25}px)"
		>
			<div
				class="text-center opacity-0 font-display {loaded ? 'animate-slide-in-blur' : ''}"
				style="animation-fill-mode: forwards;"
			>
				<h1 class="text-5xl sm:text-6xl md:text-7xl lg:text-8xl font-extrabold tracking-tight" style="line-height: 0.85;">
					<span class="block text-white drop-shadow-[0_4px_8px_rgba(0,0,0,0.4)]" style="text-shadow: 3px 3px 0 rgba(255,180,130,0.6), -1px -1px 0 rgba(255,255,255,0.2);">
						Loafy
					</span>
					<span class="block text-white drop-shadow-[0_4px_8px_rgba(0,0,0,0.4)]" style="text-shadow: 3px 3px 0 rgba(255,180,130,0.6), -1px -1px 0 rgba(255,255,255,0.2);">
						Club
					</span>
				</h1>
			</div>
		</div>

		<!-- CTA Area - positioned relative to viewport, not section -->
		<div class="absolute left-1/2 -translate-x-1/2 text-center z-20" style="top: calc(100vh - 180px);">
			<!-- Buttons with solid backgrounds -->
			<div
				class="mt-6 flex flex-col sm:flex-row items-center justify-center gap-4 opacity-0 {loaded ? 'animate-fade-in-up delay-500' : ''}"
				style="animation-fill-mode: forwards;"
			>
				<Button
					variant="gradient"
					size="lg"
					class="text-lg px-8 py-6 shadow-xl hover:shadow-2xl hover:scale-105 transition-all duration-300"
					onclick={() => goto('/sessions')}
				>
					{t('home.cta.browseSessions')}
				</Button>
				{#if !authStore.isAuthenticated}
					<Button
						size="lg"
						class="text-lg px-8 py-6 bg-white text-gray-800 hover:bg-gray-100 shadow-xl hover:shadow-2xl hover:scale-105 transition-all duration-300 border-0"
						onclick={() => goto('/auth/login')}
					>
						{t('home.cta.signIn')}
					</Button>
				{/if}
			</div>
		</div>

		<!-- Scroll Indicator - Animated Mouse Icon -->
		<div
			class="absolute left-1/2 -translate-x-1/2 opacity-0 z-10 {loaded ? 'animate-fade-in delay-1000' : ''}"
			style="animation-fill-mode: forwards; top: calc(100vh - 80px);"
		>
			<div class="flex flex-col items-center gap-2">
				<div class="mouse-scroll-icon">
					<svg width="28" height="42" viewBox="0 0 28 42" fill="none" xmlns="http://www.w3.org/2000/svg">
						<!-- Mouse body -->
						<rect x="1.5" y="1.5" width="25" height="39" rx="12.5" stroke="white" stroke-width="2.5" stroke-opacity="0.8"/>
						<!-- Scroll wheel/dot -->
						<circle class="mouse-wheel" cx="14" cy="12" r="3" fill="white" fill-opacity="0.8"/>
					</svg>
				</div>
				<span class="text-xs font-medium text-white/80 font-display" style="text-shadow: 1px 1px 2px rgba(0,0,0,0.5);">{t('home.cta.scroll')}</span>
			</div>
		</div>
	</section>

	<!-- ===== WAVE TRANSITION FROM HERO ===== -->
	<div class="relative z-30 -mt-32">
		<svg viewBox="0 -20 1440 120" fill="none" xmlns="http://www.w3.org/2000/svg" class="w-full h-28 sm:h-32" preserveAspectRatio="none">
			<!-- Wave: very flat, subtle undulation -->
			<path d="M0 25 C200 20 400 15 600 22 C800 28 1000 15 1200 20 C1350 23 1400 25 1440 25
			         L1440 120 L0 120 Z" fill="url(#wave-gradient-1)"/>
			<defs>
				<linearGradient id="wave-gradient-1" x1="0" y1="0" x2="1440" y2="0">
					<stop offset="0%" stop-color="#fef3c7"/>
					<stop offset="50%" stop-color="#fed7aa"/>
					<stop offset="100%" stop-color="#fecaca"/>
				</linearGradient>
			</defs>
		</svg>
	</div>

	<!-- ===== FEATURES SECTION (Organic Blobs) ===== -->
	<section class="relative py-20 overflow-hidden bg-gradient-to-b from-amber-100/50 via-orange-50 to-rose-50 -mt-4">
		<!-- Floating decorative elements -->
		<div class="absolute top-10 left-10 w-64 h-64 bg-yellow-200/40 rounded-full blur-3xl animate-float-slow"></div>
		<div class="absolute bottom-20 right-10 w-80 h-80 bg-orange-200/40 rounded-full blur-3xl animate-float-slower"></div>
		<div class="absolute top-1/2 left-1/3 w-40 h-40 bg-pink-200/30 rounded-full blur-2xl animate-float"></div>

		<div class="relative max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="text-center mb-16 scroll-reveal">
				<h2 class="text-4xl sm:text-5xl font-bold text-gray-800 font-display">
					{t('home.features.tagline')}
				</h2>
			</div>

			<!-- Feature Cards in organic layout -->
			<div class="grid md:grid-cols-3 gap-8 lg:gap-12">
				<!-- Card 1 -->
				<div class="scroll-reveal" style="transition-delay: 100ms;">
					<div class="relative group">
						<div class="absolute inset-0 bg-gradient-to-br from-yellow-300 to-orange-400 rounded-[2rem] blur-xl opacity-40 group-hover:opacity-60 transition-opacity duration-500 group-hover:scale-105"></div>
						<div class="relative bg-white/80 backdrop-blur-sm rounded-[2rem] p-8 shadow-lg hover:shadow-2xl transition-all duration-500 hover:-translate-y-2">
							<div class="w-16 h-16 bg-gradient-to-br from-yellow-400 to-orange-500 rounded-2xl flex items-center justify-center mb-6 shadow-lg group-hover:scale-110 transition-transform duration-300">
								<Calendar class="w-8 h-8 text-white" />
							</div>
							<h3 class="text-2xl font-bold text-gray-800 mb-3 font-display">{t('home.features.findSessions.title')}</h3>
							<p class="text-gray-600 leading-relaxed">{t('home.features.findSessions.description')}</p>
						</div>
					</div>
				</div>

				<!-- Card 2 -->
				<div class="scroll-reveal" style="transition-delay: 200ms;">
					<div class="relative group md:mt-8">
						<div class="absolute inset-0 bg-gradient-to-br from-orange-300 to-pink-400 rounded-[2rem] blur-xl opacity-40 group-hover:opacity-60 transition-opacity duration-500 group-hover:scale-105"></div>
						<div class="relative bg-white/80 backdrop-blur-sm rounded-[2rem] p-8 shadow-lg hover:shadow-2xl transition-all duration-500 hover:-translate-y-2">
							<div class="w-16 h-16 bg-gradient-to-br from-orange-400 to-pink-500 rounded-2xl flex items-center justify-center mb-6 shadow-lg group-hover:scale-110 transition-transform duration-300">
								<Zap class="w-8 h-8 text-white" />
							</div>
							<h3 class="text-2xl font-bold text-gray-800 mb-3 font-display">{t('home.features.bookInstantly.title')}</h3>
							<p class="text-gray-600 leading-relaxed">{t('home.features.bookInstantly.description')}</p>
						</div>
					</div>
				</div>

				<!-- Card 3 -->
				<div class="scroll-reveal" style="transition-delay: 300ms;">
					<div class="relative group">
						<div class="absolute inset-0 bg-gradient-to-br from-pink-300 to-rose-400 rounded-[2rem] blur-xl opacity-40 group-hover:opacity-60 transition-opacity duration-500 group-hover:scale-105"></div>
						<div class="relative bg-white/80 backdrop-blur-sm rounded-[2rem] p-8 shadow-lg hover:shadow-2xl transition-all duration-500 hover:-translate-y-2">
							<div class="w-16 h-16 bg-gradient-to-br from-pink-400 to-rose-500 rounded-2xl flex items-center justify-center mb-6 shadow-lg group-hover:scale-110 transition-transform duration-300">
								<Users class="w-8 h-8 text-white" />
							</div>
							<h3 class="text-2xl font-bold text-gray-800 mb-3 font-display">{t('home.features.beyondCourt.title')}</h3>
							<p class="text-gray-600 leading-relaxed">{t('home.features.beyondCourt.description')}</p>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- ===== CURVED DIVIDER ===== -->
	<div class="relative -mt-10 sm:-mt-12 z-10">
		<svg viewBox="0 -30 1440 130" fill="none" xmlns="http://www.w3.org/2000/svg" class="w-full h-24 sm:h-28" preserveAspectRatio="none">
			<!-- Thicker wave: organic top curve, bottom extends down with gentle wave to cover line -->
			<path d="M0 15 C180 -20 400 30 650 25 C900 20 1150 -25 1440 10
			         L1440 100 C1200 90 900 95 600 92 C300 89 100 85 0 95 Z" fill="url(#curve-gradient)"/>
			<defs>
				<linearGradient id="curve-gradient" x1="0" y1="0" x2="1440" y2="0">
					<stop offset="0%" stop-color="#fef3c7"/>
					<stop offset="50%" stop-color="#ffedd5"/>
					<stop offset="100%" stop-color="#fff1f2"/>
				</linearGradient>
			</defs>
		</svg>
	</div>

	<!-- ===== MASCOT + COMMUNITY SECTION ===== -->
	<section class="relative py-24 overflow-hidden bg-gradient-to-b from-amber-50 via-orange-50/50 to-rose-50/50 -mt-2">
		<!-- Organic blob shapes -->
		<div class="absolute -left-40 top-0 w-[500px] h-[500px] bg-gradient-to-br from-yellow-200/50 to-orange-200/50 rounded-full blur-3xl"></div>
		<div class="absolute -right-40 bottom-0 w-[400px] h-[400px] bg-gradient-to-br from-pink-200/50 to-rose-200/50 rounded-full blur-3xl"></div>

		<div class="relative max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="grid lg:grid-cols-2 gap-12 lg:gap-20 items-center">
				<!-- Mascot -->
				<div class="scroll-reveal from-left order-2 lg:order-1">
					<div class="relative">
						<!-- Decorative ring -->
						<div class="absolute inset-0 flex items-center justify-center">
							<div class="w-72 h-72 sm:w-80 sm:h-80 rounded-full border-4 border-dashed border-orange-200/60 animate-spin-very-slow"></div>
						</div>
						<div class="absolute inset-0 flex items-center justify-center">
							<div class="w-56 h-56 sm:w-64 sm:h-64 rounded-full border-2 border-dotted border-pink-200/60 animate-spin-reverse-slow"></div>
						</div>
						<img
							src="/mascot.png"
							alt="Loafy the Corgi"
							class="relative w-64 sm:w-72 mx-auto drop-shadow-2xl hover:scale-105 transition-transform duration-500 animate-float"
						/>
					</div>
				</div>

				<!-- Content -->
				<div class="scroll-reveal from-right order-1 lg:order-2" style="transition-delay: 200ms;">
					<h2 class="text-4xl sm:text-5xl font-bold text-gray-800 mb-6 font-display">
						{t('home.mascot.title')}
					</h2>
					<p class="text-xl text-gray-600 mb-8 leading-relaxed">
						{t('home.mascot.description')}
					</p>

					<!-- Stats -->
					<div class="grid grid-cols-3 gap-4 mb-8">
						<div class="text-center p-4 bg-white/60 backdrop-blur-sm rounded-2xl shadow-sm">
							<div class="text-3xl sm:text-4xl font-bold text-orange-500 font-display">50+</div>
							<div class="text-sm text-gray-500">{t('home.mascot.stats.players')}</div>
						</div>
						<div class="text-center p-4 bg-white/60 backdrop-blur-sm rounded-2xl shadow-sm">
							<div class="text-3xl sm:text-4xl font-bold text-pink-500 font-display">100+</div>
							<div class="text-sm text-gray-500">{t('home.mascot.stats.sessions')}</div>
						</div>
						<div class="text-center p-4 bg-white/60 backdrop-blur-sm rounded-2xl shadow-sm">
							<div class="text-3xl sm:text-4xl font-bold text-rose-500 font-display">∞</div>
							<div class="text-sm text-gray-500">{t('home.mascot.stats.fun')}</div>
						</div>
					</div>

					<Button
						variant="gradient"
						size="lg"
						class="text-lg px-8 py-6 shadow-xl hover:shadow-2xl hover:scale-105 transition-all duration-300"
						onclick={() => goto('/sessions')}
					>
						{t('home.mascot.joinClub')}
					</Button>
				</div>
			</div>
		</div>
	</section>

	<!-- ===== WAVE TO CTA ===== -->
	<div class="relative -mt-10 sm:-mt-12 z-10">
		<svg viewBox="0 -30 1440 130" fill="none" xmlns="http://www.w3.org/2000/svg" class="w-full h-24 sm:h-28" preserveAspectRatio="none">
			<!-- Thicker wave: different organic pattern, bottom extends down to cover line -->
			<path d="M0 -5 C200 20 400 -25 600 -15 C800 -5 1000 25 1200 -10 C1350 -20 1420 5 1440 15
			         L1440 100 C1150 92 850 98 550 95 C250 92 50 88 0 95 Z" fill="url(#cta-wave-gradient)"/>
			<defs>
				<linearGradient id="cta-wave-gradient" x1="0" y1="0" x2="1440" y2="0">
					<stop offset="0%" stop-color="#fdba74"/>
					<stop offset="50%" stop-color="#fb923c"/>
					<stop offset="100%" stop-color="#f472b6"/>
				</linearGradient>
			</defs>
		</svg>
	</div>

	<!-- ===== CTA SECTION ===== -->
	<section class="relative pt-20 pb-28 sm:pt-24 sm:pb-32 overflow-hidden bg-gradient-to-r from-orange-400 via-orange-500 to-pink-500 -mt-2">
		<!-- Decorative circles -->
		<div class="absolute top-0 left-1/4 w-64 h-64 bg-white/10 rounded-full blur-2xl"></div>
		<div class="absolute bottom-0 right-1/4 w-80 h-80 bg-white/10 rounded-full blur-2xl"></div>

		<div class="relative max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
			<div class="scroll-reveal">
				<h2 class="text-4xl sm:text-5xl font-bold text-white mb-6 font-display">
					{t('home.ctaSection.title')}
				</h2>
				<p class="text-xl text-white/90 mb-10 max-w-2xl mx-auto">
					{t('home.ctaSection.description')}
				</p>
				<div class="flex flex-col sm:flex-row items-center justify-center gap-4">
					<Button
						size="lg"
						class="text-lg px-10 py-7 bg-white text-orange-600 hover:bg-white/90 shadow-2xl hover:shadow-3xl hover:scale-105 transition-all duration-300"
						onclick={() => goto('/sessions')}
					>
						{t('home.ctaSection.browseSessions')}
					</Button>
					{#if !authStore.isAuthenticated}
						<Button
							size="lg"
							class="text-lg px-10 py-7 bg-transparent border-2 border-white text-white hover:bg-white/20 hover:text-white shadow-xl hover:shadow-2xl hover:scale-105 transition-all duration-300"
							onclick={() => goto('/auth/login')}
						>
							{t('home.ctaSection.createAccount')}
						</Button>
					{/if}
				</div>
			</div>
		</div>
	</section>

	<!-- ===== FOOTER WAVE ===== -->
	<div class="relative -mt-10 sm:-mt-12 z-10">
		<svg viewBox="0 -25 1440 120" fill="none" xmlns="http://www.w3.org/2000/svg" class="w-full h-20 sm:h-24" preserveAspectRatio="none">
			<!-- Thicker wave: unique asymmetric pattern, bottom extends down to cover line -->
			<path d="M0 10 C120 -15 320 25 520 -10 C720 -20 920 20 1120 0 C1320 -15 1400 10 1440 15
			         L1440 95 C1200 88 900 92 600 90 C300 88 100 85 0 90 Z" fill="#1f2937"/>
		</svg>
	</div>

	<!-- ===== FOOTER ===== -->
	<footer class="bg-gray-800 py-12 -mt-2">
		<div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
			<FooterContent />
		</div>
	</footer>
</div>

<style>
	/* Scroll reveal animations */
	:global(.scroll-reveal) {
		opacity: 0;
		transform: translateY(2rem);
		transition: opacity 0.7s ease-out, transform 0.7s ease-out;
	}
	:global(.scroll-reveal.visible) {
		opacity: 1;
		transform: translateY(0);
	}
	:global(.scroll-reveal.from-left) {
		transform: translateX(-2rem);
	}
	:global(.scroll-reveal.from-left.visible) {
		transform: translateX(0);
	}
	:global(.scroll-reveal.from-right) {
		transform: translateX(2rem);
	}
	:global(.scroll-reveal.from-right.visible) {
		transform: translateX(0);
	}

	/* Floating animations */
	@keyframes float {
		0%, 100% { transform: translateY(0); }
		50% { transform: translateY(-15px); }
	}
	@keyframes float-slow {
		0%, 100% { transform: translateY(0) translateX(0); }
		50% { transform: translateY(-20px) translateX(10px); }
	}
	@keyframes float-slower {
		0%, 100% { transform: translateY(0) translateX(0); }
		50% { transform: translateY(-25px) translateX(-15px); }
	}

	:global(.animate-float) {
		animation: float 4s ease-in-out infinite;
	}
	:global(.animate-float-slow) {
		animation: float-slow 8s ease-in-out infinite;
	}
	:global(.animate-float-slower) {
		animation: float-slower 12s ease-in-out infinite;
	}

	/* Speech bubble entrance animation */
	@keyframes bubble-pop-in {
		0% {
			opacity: 0;
			transform: scale(0.8) translateY(10px);
		}
		60% {
			opacity: 1;
			transform: scale(1.05) translateY(-2px);
		}
		100% {
			opacity: 1;
			transform: scale(1) translateY(0);
		}
	}

	:global(.animate-bubble-in) {
		animation: bubble-pop-in 0.5s ease-out 1.5s forwards;
	}
	:global(.animate-bubble-in-delay-1) {
		animation: bubble-pop-in 0.5s ease-out 2s forwards;
	}
	:global(.animate-bubble-in-delay-2) {
		animation: bubble-pop-in 0.5s ease-out 2.5s forwards;
	}
	:global(.animate-bubble-in-delay-3) {
		animation: bubble-pop-in 0.5s ease-out 3s forwards;
	}

	/* Speech bubble idle animation */
	@keyframes bubble-bob {
		0%, 100% {
			transform: translateY(0);
		}
		50% {
			transform: translateY(-4px);
		}
	}

	:global(.animate-bubble) {
		animation: bubble-bob 5s ease-in-out infinite;
	}
	:global(.animate-bubble-delay-1) {
		animation: bubble-bob 5.5s ease-in-out infinite;
		animation-delay: 1s;
	}
	:global(.animate-bubble-delay-2) {
		animation: bubble-bob 4.8s ease-in-out infinite;
		animation-delay: 2s;
	}
	:global(.animate-bubble-delay-3) {
		animation: bubble-bob 5.2s ease-in-out infinite;
		animation-delay: 2.5s;
	}

	/* Mouse scroll indicator animation */
	@keyframes mouse-scroll {
		0% {
			opacity: 1;
			transform: translateY(0);
		}
		50% {
			opacity: 0.5;
			transform: translateY(10px);
		}
		100% {
			opacity: 0;
			transform: translateY(16px);
		}
	}

	:global(.mouse-scroll-icon) {
		filter: drop-shadow(0 2px 4px rgba(0,0,0,0.3));
	}

	:global(.mouse-wheel) {
		animation: mouse-scroll 1.5s ease-out infinite;
	}

	/* Spinning decorative elements */
	@keyframes spin-very-slow {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}
	@keyframes spin-reverse-slow {
		from { transform: rotate(360deg); }
		to { transform: rotate(0deg); }
	}

	:global(.animate-spin-very-slow) {
		animation: spin-very-slow 30s linear infinite;
	}
	:global(.animate-spin-reverse-slow) {
		animation: spin-reverse-slow 25s linear infinite;
	}
</style>
