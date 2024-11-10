import { redirect, type Cookies } from '@sveltejs/kit';
import { dev } from '$app/environment';

import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async (event) => {
	const refresh_token = event.cookies.get('refresh_token');
	const expiration = event.cookies.get('refresh_token_expiration');
	const currentTime = Math.ceil(Date.now() / 1000);
	if (refresh_token && expiration && Number(expiration) > currentTime) {
		return redirect(302, '/');
	}
};
export const actions = {
	login: async ({ fetch, cookies, request }) => {
		const data = await request.formData();
		const password = data.get('password') as string;
		const res = await fetch('/axum-api/auth/sign-in', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(password)
		});
		if (!res.ok) {
			return { error: res.statusText };
		} else {
			const { refresh, access }: AuthTokens = await res.json();
			setCookies(cookies, 'refresh_token', refresh!.token);
			setCookies(cookies, 'refresh_token_expiration', refresh!.expiration.toString());
			setCookies(cookies, 'access_token', access!.token);
			setCookies(cookies, 'access_token_expiration', access!.expiration.toString());
			return redirect(303, '/');
		}
	},
	logout: async ({ cookies }) => {
		cookies.delete('refresh_token', { path: '/' });
		cookies.delete('refresh_token_expiration', { path: '/' });
		cookies.delete('access_token', { path: '/' });
		cookies.delete('access_token_expiration', { path: '/' });
		return redirect(303, '/auth');
	}
};
function setCookies(cookies: Cookies, cookieName: string, cookieValue: string) {
	cookies.set(cookieName, cookieValue, {
		path: '/',
		httpOnly: true,
		secure: false,
		sameSite: 'strict'
	});
}
