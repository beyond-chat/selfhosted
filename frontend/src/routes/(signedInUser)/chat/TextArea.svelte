<script lang="ts">
	import { scale } from 'svelte/transition';
	import { page } from '$app/stores';
	import MdiArrowTopThick from '~icons/mdi/arrow-top-thick';
	import FontistoPaperclip from '~icons/fontisto/paperclip';
	import IcBaselineStopCircle from '~icons/ic/baseline-stop-circle';
	import FluentSettingsChat16Filled from '~icons/fluent/settings-chat-16-filled';

	let { llmSavedSettings }: { llmSavedSettings: LLMSavedSettings } = $props();

	import { submitQuery, textArea } from './textArea.svelte';
	import { esClose_SaveDb } from './[chatId]/chatResponse';
	import { chatState } from './[chatId]/chat.svelte';

	let chatId = $derived($page.url.pathname.split('/').pop());
	let isSendButtonDisabled = $derived(textArea.value.trim() === '');

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter' && !event.shiftKey) {
			event.preventDefault();
			if (textArea.value.trim() === '') {
				return; // If the textarea is empty, don't submit
			}
			submitQuery(chatId);
		}
	}
	function handleSubmit(event: any) {
		event.preventDefault();
		submitQuery(chatId);
	}

	function handleStopGeneratingResponse(event: any) {
		event.preventDefault();
		esClose_SaveDb(chatId);
	}

	let selectedFav = $derived.by(() => {
		const favModel = llmSavedSettings.favModels.find((model) => model.id === textArea.activeFav);
		if (favModel) {
			return {
				api: llmSavedSettings.llmApiModels.find((api) => api.id === favModel.api_id),
				model: favModel.model,
				prompt: llmSavedSettings.promptConfigs.find((prompt) => prompt.id === favModel.prompt_id)
			};
		} else {
			return {
				api: undefined,
				model: undefined,
				prompt: undefined
			};
		}
	});
</script>

<div class="flex flex-col">
	<div class="textarea textarea-bordered textarea-md rounded-badge bg-base-200">
		<div class="flex w-full items-end gap-2 text-base">
			<label for="upload_file_icon" class="btn btn-circle btn-ghost btn-sm text-accent">
				<FontistoPaperclip class="h-6 w-6" />
			</label>
			<input type="file" id="upload_file_icon" class="hidden" />
			<textarea
				name="query"
				onkeydown={handleKeydown}
				bind:value={textArea.value}
				style="min-height: 1lh; max-height: 8lh; field-sizing:content"
				class="mb-[3px] w-full resize-none overflow-auto bg-transparent outline-none"
				placeholder="Enter your query!"
			>
			</textarea>
			{#if !isSendButtonDisabled}
				<button
					onclick={handleSubmit}
					class="btn btn-circle btn-accent btn-sm"
					transition:scale={{ delay: 10, duration: 400 }}
					><MdiArrowTopThick class="h-7 w-7" /></button
				>
			{:else if chatState.isResponseGenerating}
				<button
					onclick={handleStopGeneratingResponse}
					class="btn btn-circle btn-sm"
					transition:scale={{ delay: 10, duration: 400 }}
					><IcBaselineStopCircle class="h-7 w-7" /></button
				>
			{/if}
		</div>
	</div>
	<div
		class="pointer-events-none mx-auto flex w-5/6 items-center justify-between rounded-md bg-base-300 px-2 text-xs text-gray-500"
	>
		<!-- Show Api Name and Model by getting from Id -->
		<div class="flex max-w-[47%] items-center gap-2 truncate">
			<span>API: {selectedFav.api?.name}</span>
			<span>Model: {selectedFav.model}</span>
		</div>

		<div class="mx-1">
			<FluentSettingsChat16Filled class="h-[14px] w-[14px] text-accent text-opacity-70" />
		</div>
		<div class="flex max-w-[47%] items-center gap-2 truncate">
			<span>Max Tokens: {selectedFav.prompt?.max_tokens}</span>
			<span>Temp: {selectedFav.prompt?.temperature}</span>
		</div>
	</div>
</div>
