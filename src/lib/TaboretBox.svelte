<script>
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from "svelte";
    import { Ausstattung } from '../routes/store.js';

	export let listen = [];

	const oeffnen = async (pfad) => {
		await invoke("anwendung_oeffnen", {
			exec: "taboret",
			path: pfad
		}).catch((e) => {
			console.warn(e);
			Ausstattung.update((a) => { a.meldungen.push(e); return a; });
		})

	}
</script>

{#if listen.length > 0 }
<!-- <header>
	{input.name}
</header> -->
<!-- <div class="app_repos"> -->
	{#each listen as workspace}
		<div class="workspaces grid-item">
			<div class="content">
			{#if workspace.liste.length > 0}
				<header>{workspace.name}</header>
				<div class="list">
					{#each workspace.liste as repo}
						<div on:click={oeffnen(repo.pfad)}>
							{repo.name}
						</div>
					{/each}
				</div>

			{/if}
			</div>
		</div>
	{/each}
<!-- </div> -->
{/if}

<style lang="scss">
.app_repos {
	display: flex;
	flex-direction: row;
	flex-wrap: wrap;
	gap: 1rem;
}
.workspaces {
	/* width: fit-content; */
	height: fit-content;
	padding: 0px;
	/* margin: .5rem; */
	min-width: 11rem;
	max-width: 40rem;
	color: white;
	.content {
		> header {
			padding: .5rem 1rem;
			border-bottom: 2px solid #999;
		}
		.list {
			> div {
				padding: .5rem 1rem;
				cursor: pointer;
				&:hover {
					background-color: #666;
				}
				&:last-child {
					border-radius: 0px 0px 1rem 1rem;
				}
				&:before {
					/* content: "-"; */
					padding: .2rem;
				}
			}
		}
	}

}
</style>
