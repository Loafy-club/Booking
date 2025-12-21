// Disable SSR for the callback page - OAuth must be handled client-side
export const ssr = false;

// Pass URL params to the page
export function load({ url }) {
	return {
		code: url.searchParams.get('code'),
		error: url.searchParams.get('error'),
		error_description: url.searchParams.get('error_description')
	};
}
