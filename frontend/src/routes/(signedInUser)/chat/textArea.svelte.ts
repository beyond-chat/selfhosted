import { sidebar } from '$/components/layout/sidebar.svelte';
import { goto } from '$app/navigation';
import type { HistoryChatDetails } from '$lib/types/chat';
import { chatState } from './[chatId]/chat.svelte';
import { generateResponse } from './[chatId]/chatResponse';

function createTextArea() {
	let textAreaValue = $state('');
	const favModels = [0, 1, 2, 3];
	let activeFav: number = $state(0);

	return {
		get value() {
			return textAreaValue;
		},
		set value(value) {
			textAreaValue = value;
		},
		get favModels() {
			return favModels;
		},
		get activeFav() {
			return activeFav;
		},
		set activeFav(value) {
			activeFav = value;
		}
	};
}
export const textArea = createTextArea();

export async function submitQuery(chatId: any) {
	if (chatId === 'new') {
		const access_token = await (await fetch('/hooks_fetchHandler')).text();
		const res = await fetch(`/axum-api/chat/new`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${access_token}`
			},
			body: JSON.stringify({ text: textArea.value })
		});
		if (res.ok) {
			const { id, title }: HistoryChatDetails = await res.json();
			chatId = id;
			sidebar.addUnstarredChatToHistory({ id, title });
			goto(`/chat/${chatId}`);
		}
	}
	chatState.addQuery(textArea.value);
	textArea.value = '';
	generateResponse(chatState.qr[chatState.qr.length - 1].user_query, chatId);
}
