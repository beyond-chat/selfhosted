import { chatState } from './chat.svelte';
import type { ChatMessage } from '$lib/types/chat';
import { browser } from '$app/environment';
import { textArea } from '../textArea.svelte';

let eventSource: EventSource;
export async function generateResponse(query: any, chatId: any) {
	chatState.isResponseGenerating = true;
	const access_token = await (await fetch('/hooks_fetchHandler')).text();
	if (browser) {
		eventSource = new EventSource(
			`/axum-api/chat-sse?chat_id=${chatId}&access_token=${access_token}&selected_fav_model_id=${textArea.activeFav}&query=${encodeURIComponent(query)}`
		);
		eventSource.onmessage = async (event) => {
			if (event.data === 'End of Stream') {
				await esClose_SaveDb(chatId);
				return;
			}
			chatState.addResponse(event.data);
		};
		eventSource.onerror = async () => {
			await esClose_SaveDb(chatId);
		};
	}
}

export async function esClose_SaveDb(chatId: any) {
	const access_token = await (await fetch('/hooks_fetchHandler')).text();
	if (eventSource && chatState.isResponseGenerating) {
		eventSource.close();
		chatState.isResponseGenerating = false;
		const currentMessage: ChatMessage = chatState.qr[chatState.qr.length - 1];
		const res = await fetch(`/axum-api/chat/${chatId}`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${access_token}`
			},
			body: JSON.stringify(currentMessage)
		});
		if (!res.ok) {
			throw new Error(await res.text());
		}
	}
}
