<script>
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from "svelte";

	export let links = [];
	let text;

	onMount( async () => {
		links = await invoke('csv_lesen', { fach: "Suche"}).catch((err) =>  {
			console.warn(err);
		});
	});

	const suche = async(pfad) => {
		let url = pfad.replace(/%s/, text);
		invoke("app_start", { name: url });
	}
</script>

{#if links.length > 0 }
	{#each links as linkBox }
		<div class="linkliste grid-item">
			<header>{linkBox.name}</header>
			<div class="array">
				{#each linkBox.liste as app}
					<div class="app" on:click={suche(app.pfad)}>{app.name}</div>
				{/each}
			</div>
			<footer>
				<input type="text" bind:value={text} />
			</footer>
		</div>
	{/each}
{/if}

<style lang="scss">
$hellefarbe: #dd30a0;
$dunklefarbe: #7342d9;

.linkliste {
	/* margin: ; */
	padding: 0px;
	min-width: 5rem;
	/* width: fit-content; */
	height: fit-content;
	max-width: 30rem;
	border: 2px solid $hellefarbe;
	border-radius: 1rem;
	background-color: $dunklefarbe;
	border-color: $hellefarbe;
	> header {
		box-sizing: border-box;
		padding: .5rem;
		/* margin-bottom: .4rem; */
		text-align: center;
		text-transform: capitalize;
		font-weight: 800;
		/* font-size: .9rem; */
		color: #eee;
		border-bottom: 2px solid $hellefarbe;
    }
    > .array {
		> .app {
			padding: .5rem .8rem;
			cursor: pointer;
			background-color: $dunklefarbe;
			color: #eee;
			&:hover {
				background-color: $hellefarbe;
				color: #333;
			}
		}
	}
	> footer {
		border-radius: 0rem 0rem .8rem .8rem;
		input {
			width: 100%;
			margin: 0px;
			border-radius: 0rem 0rem .8rem .8rem;
		}
	}
}
</style>
