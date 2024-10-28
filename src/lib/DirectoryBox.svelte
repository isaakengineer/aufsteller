<script>
	import { invoke } from '@tauri-apps/api/core';
	import { onMount, createEventDispatcher } from "svelte";
	const dispatch = createEventDispatcher()
	let input = {}
	onMount( async () => {
		let config = await invoke("dashboard_config_load", {
			name: "directory.json"
		}).then((data) => {
			return data
		}).catch((err) => {
			console.log(err)
		});
		input = JSON.parse(config);
	});
	import Directory from '$lib/icon/Directory.svelte';



</script>

{#if input.hasOwnProperty('opener')}
	{#each Object.keys(input.workspaces) as workspace}
		<div class="faechern grid-item">
			{#if input.workspaces[workspace].length > 0}
				<div class="accordion" on:click={(e) => {

					let src = e.srcElement;
					let panel = src.nextElementSibling;
					src.classList.toggle("active");
					if (panel.style.maxHeight) {
						panel.style.maxHeight = null;
					} else {
						panel.style.maxHeight = panel.scrollHeight + "px";
					}
					dispatch('masonryRefresh')
					console.log()}}>
					{workspace}
				</div>

				<div class="panel">
					{#each input.workspaces[workspace] as dir}
						<div class="directory"
							on:click={() => {invoke("directory_open", {
								opener: input.opener,
								path: dir.path
							});
							}}>
							<div class="icon"><Directory /></div>
							<div class="name">{dir.name}</div>
							<div class="path">{dir.path}</div>
						</div>
					{/each}
				</div>
				{/if}
			</div>
	{/each}
{/if}

<style lang="scss">
.faechern {
	margin-top: .5rem;
	background-color: lightyellow !important;
	border-color: orangered !important;
	max-width: 30rem;
	> .accordion {
		color: orangered;
	}
}
.directory {

	display: grid;
	grid-template-columns: 2.5rem auto;
	grid-template-rows: 1fr 1fr;
	padding: .4rem;
	&:hover {
		cursor: pointer;
		background-color: rgb(231, 229, 229);
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

 /* Style the buttons that are used to open and close the accordion panel */
.accordion {
	box-sizing: border-box;
  cursor: pointer;
  width: 100%;
  text-align: left;
  border: none;
  outline: none;
  transition: 0.4s;

	&:after {
		content: '\02795'; /* Unicode character for "plus" sign (+) */
		font-size: 1rem;
		// color: #777;
		float: right;
		margin-left: 5px;
	}

}
:global(.accordion.active:after) {
	content: "\2796" !important; /* Unicode character for "minus" sign (-) */
}
/* Add a background color to the button if it is clicked on (add the .active class with JS), and when you move the mouse over it (hover) */
.active, .accordion:hover {
  background-color: #ccc;

}

/* Style the accordion panel. Note: hidden by default */
.panel {
  // padding: 0 18px;
  background-color: white;
  max-height: 0;
  overflow: hidden;
  /* transition: max-height 0.2s ease-out; */
}
</style>
