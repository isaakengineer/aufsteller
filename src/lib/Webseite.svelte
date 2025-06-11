<script>
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from "svelte";

	export let links = [];
</script>

<!-- <div> -->
	{#if links.length > 0 }
		{#each links as linkBox }
			<div class="linkliste grid-item">
				<div class="content">
				<header>{linkBox.name}</header>
				<div class="array">
					{#each linkBox.liste as app}
						<div class="app" on:click={() => {
							invoke("app_start", {name: app.pfad});
						}}>{app.name}</div>
					{/each}
				</div>
				</div>
			</div>
		{/each}
	{/if}
<!-- </div> -->

<style lang="scss">
.linkliste {
	/* margin: ; */
	padding: 0px;
	min-width: 5rem;
	/* width: fit-content; */
	height: fit-content;
	max-width: 30rem;
	border: 2px solid #eee;
	border-radius: 1rem;
	background-color: #2832c2;
	border-color: lightsteelblue;
	.content {
		> header {
			box-sizing: border-box;
			padding: .5rem;
			/* margin-bottom: .4rem; */
			text-align: center;
			text-transform: capitalize;
			font-weight: 800;
			/* font-size: .9rem; */
			color: #eee;
			border-bottom: 2px solid lightsteelblue;
    }
    > .array {
			> .app {
				padding: .5rem .8rem;
				cursor: pointer;
				background-color: #2832c2;
				color: #eee;
				&:hover {
					background-color: lightskyblue;
					color: #333;
				}

			}
		}
		*:last-child {
			border-radius: 0rem 0rem .8rem .8rem;
		}
	}

}
</style>
