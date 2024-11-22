<script>
	import { invoke } from '@tauri-apps/api/core';
	import { onMount, createEventDispatcher } from "svelte";
	const dispatch = createEventDispatcher()
	let input = {}
	let listen = [];
	onMount( async () => {
		listen = await invoke('csv_lesen', { fach: "Dir"}).catch((err) =>  {
			console.warn(err);
		});
	});
	import Directory from '$lib/icon/Directory.svelte';
</script>

{#if listen.length > 0 }
	{#each listen as workspace}
		<div class="faechern grid-item span-2">
			{#if workspace.liste.length > 0}
				<header>{ workspace.name }</header>
				<div class="panel">
					{#each workspace.liste as dir}
						<div class="directory"
							on:click={() => {invoke("directory_open", {
								opener: "nautilus",
								path: dir.pfad
							});
							}}>
							<div class="icon"><Directory /></div>
							<div class="name">{dir.name}</div>
							<div class="path">{dir.pfad}</div>
						</div>
					{/each}
				</div>
				{/if}
			</div>
	{/each}
{/if}

<style lang="scss">
.faechern {
	padding: 0px !important;
	display: flex;
	flex-direction: column;
	/* margin-top: .5rem; */
	background-color: lightyellow !important;
	border-color: orangered !important;
	> header {
		box-sizing: border-box;
		padding: .5rem;
		border-bottom: 2px solid orangered;
		font-weight: 700;
	}
	overflow: hidden;
	> .panel {
		max-height: 100%;
		overflow: scroll;
		display: flex;
		flex-direction: column;
		gap: .25rem;
		> .directory {
			display: grid;
			grid-template-columns: 2.5rem auto;
			grid-template-rows: 1fr 1fr;
			padding: .25rem;
			&:hover {
				cursor: pointer;
				background-color: lightgoldenrodyellow;
			}
			> .name {
				font-size: .9rem;
			}
			> .path {
				color: gray;
				font-size: .7rem;
				grid-column-start: 2;
			}
			> .icon {
				width: 1.5rem;
				height: 1.5rem;
				float: left;
				padding: .5rem;
				grid-row-start: 1;
				grid-row-end: 3;
			}
		}
	}
}

</style>
