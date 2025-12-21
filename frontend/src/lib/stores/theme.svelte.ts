type Theme = 'light' | 'dark' | 'system';

class ThemeStore {
	theme = $state<Theme>('system');
	resolvedTheme = $state<'light' | 'dark'>('light');

	constructor() {
		// Initialize will be called from layout after mount
	}

	initialize() {
		// Load saved theme preference
		const savedTheme = localStorage.getItem('loafy_theme') as Theme | null;
		if (savedTheme && ['light', 'dark', 'system'].includes(savedTheme)) {
			this.theme = savedTheme;
		}

		// Apply theme
		this.applyTheme();

		// Listen for system preference changes
		if (typeof window !== 'undefined') {
			const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
			mediaQuery.addEventListener('change', () => {
				if (this.theme === 'system') {
					this.applyTheme();
				}
			});
		}
	}

	setTheme(newTheme: Theme) {
		this.theme = newTheme;
		localStorage.setItem('loafy_theme', newTheme);
		this.applyTheme();
	}

	private applyTheme() {
		if (typeof document === 'undefined') return;

		let isDark = false;

		if (this.theme === 'system') {
			isDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
		} else {
			isDark = this.theme === 'dark';
		}

		this.resolvedTheme = isDark ? 'dark' : 'light';

		if (isDark) {
			document.documentElement.classList.add('dark');
		} else {
			document.documentElement.classList.remove('dark');
		}
	}
}

export const themeStore = new ThemeStore();
