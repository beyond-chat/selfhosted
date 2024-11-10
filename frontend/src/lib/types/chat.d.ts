import type { UUID } from 'crypto';

// Chat history
type HistoryChatDetails = {
	id: UUID;
	title: string;
};
type UnstarredGroupedHistory = {
	time_period: string;
	period_chats: HistoryChatDetails[];
};
type ChatHistory = {
	unstarred_history: UnstarredGroupedHistory[];
	starred_history: HistoryChatDetails[];
};

// Each chat
type ChatDetails = {
	id: UUID;
	title: string;
	starred: boolean;
};
type ChatMessage = {
	user_query: string;
	assistant_response: string;
};
type Chat = {
	details: ChatDetails;
	messages: ChatMessage[];
};
