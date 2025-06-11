<script>
	import { invoke } from "@tauri-apps/api/core";
	import { onMount } from "svelte";
	import { Ausstattung } from "../routes/store.js";

	export let listen = [];

	const oeffnen = async (pfad) => {
		await invoke("anwendung_oeffnen", {
			exec: "open",
			path: pfad,
		}).catch((e) => {
			console.warn(e);
			Ausstattung.update((a) => {
				a.meldungen.push(e);
				return a;
			});
		});
	};

	import Directory from "$lib/icon/Directory.svelte";
</script>

{#if listen.length > 0}
	{#each listen as workspace}
		<div class="faechern grid-item span-2">
			{#if workspace.liste.length > 0}
				<header>{workspace.name}</header>
				<div class="panel">
					{#each workspace.liste as dir}
						<div class="directory" on:click={oeffnen(dir.pfad)}>
							<!-- <div class="icon"><Directory /></div> -->
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
		height: fit-content;
		padding: 0px !important;
		display: flex;
		flex-direction: column;
		/* margin-top: .5rem; */
		background-color: lightyellow !important;
		border-color: orangered !important;
		max-width: 18rem;
		> header {
			box-sizing: border-box;
			padding: 0.5rem;
			border-bottom: 2px solid orangered;
			font-weight: 700;
		}
		overflow: hidden;
		> .panel {
			max-height: 100%;
			overflow: auto;
			display: flex;
			flex-direction: column;
			gap: 1px;
			background-color: orange;
			> .directory {
				/* display: grid; */
				/* grid-template-columns: 2.5rem auto; */
				/* grid-template-rows: 1fr 1fr; */
				padding: 0.5rem;
				background-color: lightyellow;
				&:hover {
					cursor: pointer;
					background-color: lightgoldenrodyellow;
				}
				> .name {
					font-size: 0.9rem;
				}
				> .path {
					color: gray;
					font-size: 0.6rem;
					grid-column-start: 2;
					text-align: end;
					color: darkred;
				}
				> .icon {
					width: 1.5rem;
					height: 1.5rem;
					float: left;
					padding: 0.5rem;
					grid-row-start: 1;
					grid-row-end: 3;
				}
			}
		}
	}
</style>
