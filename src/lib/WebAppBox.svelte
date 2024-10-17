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

{#if links}
	<header>
		<div>Apps</div>
	</header>
	{#each Object.entries(links) as [name, pack] }
		<div class="list_app">
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

<style lang="scss">
	 .list_app {
    padding: 1rem;


    > header {
			margin-bottom: .4rem;
			text-align: end;
      text-transform: capitalize;
      font-weight: 300;
			font-size: .9rem;
    }
		> .array {
			display: flex;
    	flex-wrap: wrap;
			gap: 1rem;
			> .app {
				padding: .5rem .8rem;
				background-color: rgba(30, 30, 30, .75);
				color: white;
				// border: 2px solid gray;
				border-radius: 4px;
				cursor: pointer;
				&:hover {
					background-color: rgba(30, 30, 30, 1);
					color: white;
					// border: 2px solid black;
				}
			}
		}

  }
</style>
