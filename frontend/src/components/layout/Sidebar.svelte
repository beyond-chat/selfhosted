<script lang="ts">
	import F7ArrowLeftToLine from '~icons/f7/arrow-left-to-line';
	import MaterialSymbolsChatAddOn from '~icons/material-symbols/chat-add-on';

	import ProfileDropdown from './ProfileDropdown.svelte';
	import { sidebar } from './sidebar.svelte';
	import type { ChatHistory } from '$lib/types/chat';
	import { page } from '$app/stores';

	let isChatPage = $derived($page.url.pathname.startsWith('/chat'));

	let { chats }: { chats: ChatHistory } = $props();

	// onMount(fetchChatHistory)
	let promise = $state(fetchChatHistory());
	async function fetchChatHistory() {
		if (sidebar.chatHistory.unstarred_history.length === 0) {
			sidebar.initHistory(chats);
		}
	}
</script>

<div class="custom_border flex h-full flex-col gap-6 px-2">
	<div class="syncSidebarTopbar_Alignment flex flex-nowrap items-center justify-between">
		<a href="/" class="px-2 text-xl font-semibold">Beyond Chat</a>
		{#if isChatPage}
			<div class="tooltip tooltip-left" data-tip="Close Sidebar">
				<button class="btn btn-square btn-ghost btn-sm" onclick={sidebar.toggle}>
					<F7ArrowLeftToLine class="h-6 w-6" />
				</button>
			</div>
		{/if}
	</div>
	<a
		href="/chat/new"
		class="btn btn-ghost btn-sm flex items-center justify-start gap-1 px-2 text-xl text-accent opacity-90"
	>
		<MaterialSymbolsChatAddOn class="h-6 w-6" />
		<h3>Start a new chat</h3>
	</a>
	{#await promise}
		<!-- promise is pending -->
		<p>Loading...</p>
	{:then value}
		<!-- promise was fulfilled -->
		<div>
			<p class="my-1 px-1 text-sm font-bold opacity-50">Starred Chats</p>
			<ul class="max-h-[25vh] overflow-auto">
				{#each sidebar.chatHistory.starred_history as chat}
					<li class="truncate">
						<a
							href="/chat/{chat.id}"
							class="btn btn-ghost btn-sm w-full justify-start font-sans"
							class:activeChat={chat.id === $page.params.chatId}>{chat.title}</a
						>
					</li>
				{/each}
			</ul>
		</div>
		<ul class="flex-1 overflow-auto">
			{#each sidebar.chatHistory.unstarred_history as period_group}
				<li class="my-1 px-1 text-sm font-bold opacity-40">{period_group.time_period}</li>
				<ul>
					{#each period_group.period_chats as chat}
						<li class="truncate">
							<a
								href="/chat/{chat.id}"
								class="btn btn-ghost btn-sm w-full justify-start font-sans"
								class:activeChat={chat.id === $page.params.chatId}>{chat.title}</a
							>
						</li>
					{/each}
				</ul>
			{/each}
		</ul>
	{/await}
	<ProfileDropdown />
</div>

<style lang="postcss">
	.custom_border {
		border-right-width: 1px;
		border-right-color: var(--fallback-bc, oklch(var(--bc) / 0.2));
	}

	.activeChat {
		@apply bg-emerald-800 bg-opacity-80;
	}
</style>
