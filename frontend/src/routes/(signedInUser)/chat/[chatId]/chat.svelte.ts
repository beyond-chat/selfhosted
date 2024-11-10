import type { ChatMessage } from '$lib/types/chat';

function createChatState() {
	let qr: ChatMessage[] = $state([]);
	let starred = $state(false);
	let title = $state('');
	let isResponseGenerating = $state(false);

	function emptyQR() {
		qr = [];
	}
	function addQuery(user_query: string) {
		qr.push({ user_query, assistant_response: '' });
	}
	function addResponse(response: string) {
		qr[qr.length - 1].assistant_response += response;
	}
	return {
		get qr() {
			return qr;
		},
		get starred() {
			return starred;
		},
		set starred(value) {
			starred = value;
		},
		get title() {
			return title;
		},
		set title(value) {
			title = value;
		},
		get isResponseGenerating() {
			return isResponseGenerating;
		},
		set isResponseGenerating(value) {
			isResponseGenerating = value;
		},
		emptyQR,
		addQuery,
		addResponse
	};
}
export const chatState = createChatState();
