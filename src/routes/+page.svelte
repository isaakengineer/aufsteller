<script>
	import { invoke, convertFileSrc } from '@tauri-apps/api/core';
	import Masonry from 'masonry-layout';
	import { onMount } from 'svelte';
	import { fade, slide, blur } from 'svelte/transition';
	import Greet from "$lib/Greet.svelte";
	export let data;

	let items = [];

	let Ausstattung = {
		album: true,
		notizen: false,
	}

	async function projectOpen(project) {
		console.log(project)
		let projects = await invoke("project_on", {name: project});
	}

	let elem;
	let msnry;
	let refreshFunc = () => {};
	let bildern = [];
	let bildernIndex = 0;
	let notizen = [];
	let notizenIndex = 0;

	onMount( async () => {
		bildern = await invoke("album_init");
		notizen = await invoke("notizen_init");

		console.log("bildern", bildern);
		let config_web = await invoke("dashboard_config_load", {
			name: "webapp.json"
		}).then((data) => {
			return data
		}).catch((err) => {
			console.log(err)
		});
		let links = JSON.parse(config_web);

		let config_dir = await invoke("dashboard_config_load", {
			name: "directory.json"
		}).then((data) => {
			return data
		}).catch((err) => {
			console.log(err)
		});
		let dirs = JSON.parse(config_dir);
		console.log("these are dirs", dirs)
		console.log("these are links", links)
		items = items.concat(Object.keys(links));
		items = items.concat(Object.keys(dirs.workspaces));
		console.log("before items", items)

		// elem = document.querySelector('#masonry-grid');
		// msnry = new Masonry( elem, {
		// 	// options
		// 	itemSelector: '.grid-item',
		// 	columnWidth: 200,
		// 	// initLayout: false
		// });
		// console.log("this masonry", msnry);

		// refreshFunc = msnry.layout;
		// refreshFunc()
	});
	console.log("these are items")
	console.log(items)

	// function draw() {
	// 	msnry = undefined;
	// 	let elem_2 = document.querySelector('#masonry-grid');
	// 	let msnry_2 = new Masonry( elem, {
	// 		// options
	// 		itemSelector: '.grid-item',
	// 		columnWidth: 200,
	// 		initLayout: false
	// 	});
	// 	msnry_2.layout()
	// 	console.log("to refresh")
	// }

	import WebAppBox from "$lib/WebAppBox.svelte";
	import MonitorSwitch from "$lib/MonitorSwitch.svelte";
	import TaboretBox from "$lib/TaboretBox.svelte";
	import DirectoryBox from "$lib/DirectoryBox.svelte";


</script>

<div class="aufsteller">
	<div class="notizen">
		{#if Ausstattung.album}
			<!-- <div class="album" transition:slide|global> -->
			<div class="album">
				<div class="bilderrahmen">
					{#key bildernIndex}
						<!-- <img in:blur|global={{ duration: 800 }} out:fade|global src={convertFileSrc(bildern[bildernIndex])} /> -->
						<img src={convertFileSrc(bildern[bildernIndex])} />
					{/key}
				</div>
				<div class="albumkontrollen">
					<button on:click={() => { bildernIndex = ((bildernIndex == 0) ? bildern.length : bildernIndex) - 1 }}>Vorherrige</button>
					<button on:click={() => { Ausstattung.notizen = true; Ausstattung.album = false; }}>Notizen</button>
					<button on:click={() => { bildernIndex = (bildernIndex + 1) % bildern.length }}>Nächste</button>
				</div>
			</div>
		{:else if Ausstattung.notizen}
			<!-- <div transition:slide|global> -->
			<div>
				<div class="notizenkontrollen">
					<button on:click={() => { Ausstattung.notizen = false; Ausstattung.album = true; }}>Album</button>
					<button on:click={() => { }}>Open</button>
				</div>
				<div class="notizrahmen">
					{#each notizen as notiz, i }
						<div class="notiz">
							<header on:click={() => { notizenIndex = i; }}>{notiz.name}</header>
							{#if notizenIndex == i}
								<pre class="inhalt">{notiz.inhalt}</pre>
							{/if}
						</div>
					{/each}
				</div>
			</div>
		{/if}
	</div>
	<div class="kommande">
		<!-- <div>Kommande Here</div> -->
	</div>
	<div class="informationen"></div>
	<div class="arbeitsumgebung">
		<header>
			<!-- <button on:click={draw}>draw</button> -->
		</header>
		<nav>
			<div>Default</div>
		</nav>
	</div>
	<div class="applikationen"></div>
	<div class="zeuge">
		<MonitorSwitch />
	</div>
	<div id="masonry-grid" class="boxen" >
		<!-- <Masonry stretchFirst={true} > -->
			<TaboretBox  />
			<!-- <DirectoryBox on:masonryRefresh={() => {setTimeout(draw(), 10000)}} /> -->
			<DirectoryBox />
			<WebAppBox />
		<!-- </Masonry> -->
    </div>
</div>

<style lang="scss">
#masonry-grid {
	display: grid;
    grid-template-columns: repeat(auto-fill, minmax(14rem, 1fr));
    grid-gap: 1px;
    grid-auto-rows: minmax(10rem, auto);
    grid-auto-flow: dense;
    padding: 1px;
	.grid-item {
		padding: 1rem;
	    font-size: 14px;
	    font-weight: bold;
	    text-transform: uppercase;
	    color: #000;
	    background-color: #ccc;
	    border-radius: 10px;
		&.span-2 {
		    grid-column-end: span 2;
			grid-row-end: span 2;
		}
		&.span-3 {
    		grid-column-end: span 3;
     		grid-row-end: span 4;
		}
	}
}
button {
	padding: .3rem .8rem;
}
.notizrahmen {
	background-color: transparent !important;
	padding: 0px !important;
	border: none !important;
	display: flex;
	flex-direction: column;
	gap: .2rem;
	> .notiz {
		background-color: lightyellow;
		box-shadow: 0px 0px .5rem rgba(0, 0, 0, .8);
		> header {
			padding: .1rem .5rem;
			border-bottom: 1px dashed yellow;
			font-size: .9rem;
			&:hover { cursor: pointer;}
		}
		> .inhalt {
			padding: .5rem 1rem;
			color: #000;
		}
	}
}
.notizen > div {
	box-shadow: none;
	background-color: transparent !important;
	border: none;
}
.notizen .notizenkontrollen {
	display: flex;
	justify-content: space-between;
}
.album {
	box-sizing: border-box;
	height: 100%;
	overflow-y: scroll;
	padding: 1.25rem !important;
	display: flex;
	flex-direction: column;
	gap: 1rem;
	justify-content: space-between;
	> .bilderrahmen {
		flex-grow: 1;
		max-height: 90%;
		overflow: hidden;
	}
	img {
		display: block;
		margin-right: auto;
		margin-left: auto;
		border-radius: .5rem;
		box-shadow: 0px 0px .5rem rgba(0, 0, 0, .8);
		max-width: 100%;
		max-height: 100%;
	}
	> .albumkontrollen {
		display: flex;
		justify-content: space-between;
	}
}
.boxen {
	display: grid;
	gap: 10px;
	grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
	grid-template-rows: masonry;
}
.aufsteller {
	width: 100vw;
	height: 100vh;
	overflow: hidden;
	display: grid;
	padding: .8rem;
	box-sizing: border-box;
	gap: .8rem;
	grid-template-areas:
		"boxen 			arbeitsumgebung zeuge"
		"boxen 			arbeitsumgebung applikationen"
		"boxen 			arbeitsumgebung notizen"
		"informationen 	informationen 	notizen"
		"informationen 	informationen 	kommande";
	grid-template-columns: 13fr 10rem 5fr;
	grid-template-rows: 1fr 5rem 1fr 4fr 5rem;
	overflow: hidden;
	> .notizen { grid-area: notizen; }
	> .kommande { grid-area: kommande; }
	> .informationen { grid-area: informationen; }
	> .arbeitsumgebung { grid-area: arbeitsumgebung; }
	> .applikationen { grid-area: applikationen; }
	> .boxen { grid-area: boxen; }
	> .zeuge { grid-area: zeuge; }
	> div {
	}
}
.aufsteller {
	> div {
		/* box-sizing: border-box;
		padding: 1rem; */

		:global(> div) {
			padding: .8rem;
			background-color: #333;
			border: 2px solid gray;
			border-radius: 1rem;
			box-shadow: 0px 0px .5rem rgba(0, 0, 0, .8);
		}
	}
}
.essential { background-color: transparent !important; }

.dashboards {
  display: flex;
  gap: 1rem;
  padding: 1rem 0px;
  overflow-y: auto;
  :global(> section) {
    border-radius: 4px;
    margin: 0px 1rem;
    padding: 1rem 1rem;
    background-color: rgba(255, 255, 255, 0.75);
    &:hover {
      background-color: rgba(255, 255, 255, 1);
    }
    :global(> header) {
      color: rgb(12, 6, 65);
      text-align: end;
      font-weight: 700;
    }
    :global(> *) {

    }
  }
}
.workspace-conductor-app {

  display: grid;
  grid-template-areas: "main drawer";
  grid-template-columns: 5fr 3fr;
  > main {

    grid-area: main;
  }
  > .drawer {
    grid-area: drawer;
    > .control-tabs {
      height: 100vh;
      overflow-y: auto;
    }
  }
}
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
button.action {
  padding: 0px;
  width: 1.8rem;
  height: 1.8rem;
}
button.action > svg {
  margin: auto auto;
  width: 1rem;
  height: 1rem;
}
</style>
