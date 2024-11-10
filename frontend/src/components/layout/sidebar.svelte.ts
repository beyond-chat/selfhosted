import type { ChatHistory, HistoryChatDetails } from '$lib/types/chat';

function createSidebar() {
	let isOpen = $state(true);
	let chatHistory: ChatHistory = $state({
		unstarred_history: [],
		starred_history: []
	});

	function toggle() {
		isOpen = !isOpen;
	}
	function initHistory(chats: ChatHistory) {
		chatHistory = chats;
	}
	function addUnstarredChatToHistory(chat: HistoryChatDetails) {
		if (chatHistory.unstarred_history.length === 0) {
			chatHistory.unstarred_history.push({
				time_period: 'Recent',
				period_chats: [chat]
			});
		} else {
			chatHistory.unstarred_history[0].period_chats.unshift(chat);
		}
	}

	return {
		get isOpen() {
			return isOpen;
		},
		toggle,
		get chatHistory() {
			return chatHistory;
		},
		initHistory,
		addUnstarredChatToHistory
	};
}

export const sidebar = createSidebar();

declare global {
	interface Array<T> {
		myShiftFilter(predicate: (value: T) => boolean): void;
	}
}
Array.prototype.myShiftFilter = function <T>(this: T[], predicate: (value: T) => boolean): void {
	let i, j;

	for (i = 0, j = 0; i < this.length; ++i) {
		if (predicate(this[i])) {
			this[j] = this[i];
			++j;
		}
	}
	while (j < this.length) {
		this.pop();
	}
};
