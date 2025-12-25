type Theme = 'light' | 'dark';

class ThemeStore {
	theme = $state<Theme>('light');

	constructor() {
		// Initialize will be called from layout after mount
	}

	initialize() {
		// Load saved theme preference
		const savedTheme = localStorage.getItem('loafy_theme') as Theme | null;
		if (savedTheme && ['light', 'dark'].includes(savedTheme)) {
			this.theme = savedTheme;
		}

		// Apply theme
		this.applyTheme();
	}

	setTheme(newTheme: Theme) {
		this.theme = newTheme;
		localStorage.setItem('loafy_theme', newTheme);
		this.applyTheme();
	}

	private applyTheme() {
		if (typeof document === 'undefined') return;

		const isDark = this.theme === 'dark';

		if (isDark) {
			document.documentElement.classList.add('dark');
		} else {
			document.documentElement.classList.remove('dark');
		}
	}
}

export const themeStore = new ThemeStore();
