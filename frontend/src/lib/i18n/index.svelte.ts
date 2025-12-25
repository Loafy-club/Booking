import en from './en.json';
import vi from './vi.json';

export type Locale = 'en' | 'vi';

type TranslationValue = string | { [key: string]: TranslationValue };
type Translations = { [key: string]: TranslationValue };

const translations: Record<Locale, Translations> = { en, vi };
const supportedLocales: Locale[] = ['en', 'vi'];

/**
 * Detect the best locale based on browser language settings
 */
function detectBrowserLocale(): Locale {
	if (typeof navigator === 'undefined') return 'en';

	// Get browser languages (ordered by preference)
	const browserLanguages = navigator.languages ?? [navigator.language];

	for (const lang of browserLanguages) {
		// Extract language code (e.g., 'vi-VN' → 'vi', 'en-US' → 'en')
		const langCode = lang.split('-')[0].toLowerCase();
		if (supportedLocales.includes(langCode as Locale)) {
			return langCode as Locale;
		}
	}

	return 'en';
}

class I18nStore {
	locale = $state<Locale>('en');

	constructor() {
		// Load saved locale from localStorage, or detect from browser
		if (typeof window !== 'undefined') {
			const saved = localStorage.getItem('loafy_language') as Locale | null;
			if (saved && supportedLocales.includes(saved)) {
				this.locale = saved;
			} else {
				// No saved preference - detect from browser language
				this.locale = detectBrowserLocale();
			}
		}
	}

	setLocale(newLocale: Locale) {
		this.locale = newLocale;
		if (typeof window !== 'undefined') {
			localStorage.setItem('loafy_language', newLocale);
		}
	}

	/**
	 * Translate a key to the current locale
	 * Supports nested keys like 'account.profile.title'
	 */
	t(key: string, params?: Record<string, string | number>): string {
		const keys = key.split('.');
		let value: TranslationValue | undefined = translations[this.locale];

		for (const k of keys) {
			if (value && typeof value === 'object' && k in value) {
				value = value[k];
			} else {
				// Fallback to English if key not found
				value = translations['en'];
				for (const fallbackKey of keys) {
					if (value && typeof value === 'object' && fallbackKey in value) {
						value = value[fallbackKey];
					} else {
						return key; // Return the key itself if not found
					}
				}
				break;
			}
		}

		if (typeof value !== 'string') {
			return key;
		}

		// Replace parameters like {name} with actual values
		if (params) {
			return value.replace(/\{(\w+)\}/g, (_, param) => {
				return params[param]?.toString() ?? `{${param}}`;
			});
		}

		return value;
	}
}

export const i18n = new I18nStore();

/**
 * Create a reactive translation function for use in Svelte components.
 * Must be called at the top level of a component to establish reactivity.
 *
 * Usage:
 * ```
 * import { useTranslation } from '$lib/i18n/index.svelte';
 * const t = useTranslation();
 * ```
 */
export function useTranslation() {
	return (key: string, params?: Record<string, string | number>): string => {
		i18n.locale; // Read locale to establish reactive dependency
		return i18n.t(key, params);
	};
}
