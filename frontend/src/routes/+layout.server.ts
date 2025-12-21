import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ locals }) => {
	// Get session from server-side Supabase client (set in hooks.server.ts)
	if (locals.safeGetSession) {
		const { session } = await locals.safeGetSession();
		return {
			session
		};
	}

	return {
		session: null
	};
};
