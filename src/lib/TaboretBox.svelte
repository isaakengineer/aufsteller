<script>
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from "svelte";
	let input = {}
	onMount( async () => {
		let config = await invoke("dashboard_config_load", {
			name: "taboret.json"
		}).then((data) => {
			console.log("this is data")
			console.log(data)
			return data
		}).catch((err) => {
			console.log(err)
		});
		input = JSON.parse(config);
		console.log(input)
	});

</script>

{#if input.hasOwnProperty('name')}
<!-- <header>
	{input.name}
</header> -->
<!-- <div class="app_repos"> -->
	{#each Object.keys(input.workspaces) as workspace}
		<div class="workspaces grid-item">
			{#if input.workspaces[workspace].length > 0}
				<header>{workspace}</header>
				<div class="list">
					{#each input.workspaces[workspace] as repo}
						<div on:click={() => {invoke("app_open", {
								exec: input.exec,
								path: repo.path
							}
							)}}>
							{repo.name}
						</div>
					{/each}
				</div>

			{/if}
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
	width: fit-content;
	height: fit-content;
	padding: 0px;
	margin: .5rem;
	min-width: 11rem;
	max-width: 40rem;
	color: white;
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
</style>
