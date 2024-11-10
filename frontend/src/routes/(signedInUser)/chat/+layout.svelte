<script lang="ts">
	import ChatTopbar from './Topbar.svelte';
	import TextArea from './TextArea.svelte';
	import { textArea } from './textArea.svelte';

	const { data, children } = $props();
	let { llmSavedSettings } = $derived(data);
</script>

<div class="mx-auto flex min-h-screen flex-col justify-between px-6">
	<div class="sticky top-0 bg-base-100">
		<div class="syncSidebarTopbar_Alignment relative flex flex-nowrap items-center justify-between">
			<ChatTopbar {llmSavedSettings} />
		</div>
	</div>

	<div class="flex flex-1 justify-center">
		<div class="flex flex-1 flex-col">
			<div class="sticky bottom-0 mt-auto bg-base-100">
				<div class="mt-4">
					<div class="flex flex-col">
						<div class="textarea textarea-md mb-4 flex flex-col items-center justify-center">
							<span class="text-base text-gray-500">Favorite Models:</span>
							<div class="join">
								{#each textArea.favModels as value, i}
									{#if llmSavedSettings.favModels.some((model) => model.id === i)}
										<button
											type="button"
											class="btn join-item btn-sm"
											class:btn-active={textArea.activeFav === value}
											onclick={() => (textArea.activeFav = value)}
										>
											{value === 0 ? 'Default' : value}
										</button>
									{:else}
										<button type="button" class="btn btn-disabled join-item btn-sm">
											{value === 0 ? 'Default' : value}
										</button>
									{/if}
								{/each}
								<input type="hidden" name="favorite" value={textArea.activeFav} />
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>

		<div class="content_chattextarea_width flex flex-col justify-between">
			<div class="flex flex-1 flex-col">
				{@render children()}
			</div>
			<div class="sticky bottom-0 bg-base-100">
				<div class="mt-4">
					<TextArea {llmSavedSettings} />
				</div>
			</div>
		</div>

		<div class="flex-1"></div>
	</div>
</div>

<style lang="postcss">
	.content_chattextarea_width {
		@apply mx-auto w-7/12 max-w-screen-lg;
	}
</style>
