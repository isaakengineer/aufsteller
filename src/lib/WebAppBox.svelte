<script>
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from "svelte";

	export let links = {};

	onMount( async () => {
		let config = await invoke("dashboard_config_load", {
			name: "webapp.json"
		}).then((data) => {
			return data
		}).catch((err) => {
			console.log(err)
		});
		links = JSON.parse(config);
	});
</script>

<!-- <div> -->
	{#if links}
		{#each Object.entries(links) as [name, pack] }
			<div class="linkliste grid-item">
				<header>{name}</header>
				<div class="array">
					{#each pack as app}
						<div class="app" on:click={() => {
								invoke("app_start", {name: app.url});
						}
						}>{app.name}</div>
					{/each}
				</div>
			</div>
		{/each}
	{/if}
<!-- </div> -->

<style lang="scss">
.linkliste {
	margin: 1rem .2rem;
	padding: 0px;
	min-width: 5rem;
	width: fit-content;
	max-width: 30rem;
	border: 2px solid #eee;
	border-radius: 1rem;
	background-color: lightsteelblue;
	border-color: #2832c2;
	> header {
		box-sizing: border-box;
		padding: .5rem;
		margin-bottom: .4rem;
		text-align: center;
		text-transform: capitalize;
		font-weight: 800;
		font-size: .9rem;
		color: #2832c2;
    }
    > .array {
		> .app {
			padding: .5rem .8rem;
			cursor: pointer;
			background-color: #2832c2;
			color: white;
			&:hover {
				background-color: rgba(30, 30, 30, 1);
				color: white;
			}
			&:last-child {
				border-radius: 0rem 0rem .8rem .8rem;
			}
		}
	}
}
</style>
