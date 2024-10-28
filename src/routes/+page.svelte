<script>
	import { invoke } from '@tauri-apps/api/core';
	import Masonry from 'masonry-layout';
	import { onMount } from 'svelte';
	import Greet from "$lib/Greet.svelte";
	export let data;

	let items = [];

	async function projectOpen(project) {
		console.log(project)
		let projects = await invoke("project_on", {name: project});
	}

	let elem;
	let msnry;
	let refreshFunc = () => {};
	onMount( async () => {
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

		elem = document.querySelector('.grid');
		msnry = new Masonry( elem, {
			// options
			itemSelector: '.grid-item',
			columnWidth: 200,
			// initLayout: false
		});
		console.log("this masonry", msnry);

		// refreshFunc = msnry.layout;
		// refreshFunc()
	});
	console.log("these are items")
	console.log(items)

	function draw() {
		msnry = undefined;
		let elem_2 = document.querySelector('.grid');
		let msnry_2 = new Masonry( elem, {
			// options
			itemSelector: '.grid-item',
			columnWidth: 200,
			initLayout: false
		});
		msnry_2.layout()
		console.log("to refresh")

	}
	import WebAppBox from "$lib/WebAppBox.svelte";
	import MonitorSwitch from "$lib/MonitorSwitch.svelte";
	import TaboretBox from "$lib/TaboretBox.svelte";
	import DirectoryBox from "$lib/DirectoryBox.svelte";


</script>

<div class="aufsteller">
	<div class="notizen"></div>
	<div class="kommande"></div>
	<div class="informationen"></div>
	<div class="arbeitsumgebung">
		<nav>
			<div>Default</div>
		</nav>
	</div>
	<div class="applikationen"></div>
	<div class="zeuge">
		<MonitorSwitch />
	</div>
	<div class="boxen grid" >
		<!-- <Masonry stretchFirst={true} > -->
			<TaboretBox />
			<DirectoryBox on:masonryRefresh={() => {

				setTimeout(draw(), 10000)}
			} />
			<WebAppBox />
		<!-- </Masonry> -->
    </div>
	<div class="drawer dashboard-container dashboard view-dashboard">
		<section>

		</section>
	</div>
</div>

<style lang="scss">
.boxen {
	display: grid;
	gap: 10px;
	grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
	grid-template-rows: masonry;
}
.aufsteller {
	width: 100vw;
	height: 100vh;
	display: grid;
	grid-template-areas:
		"boxen arbeitsumgebung zeuge"
		"boxen arbeitsumgebung applikationen"
		"boxen arbeitsumgebung notizen"
		"informationen informationen notizen"
		"informationen informationen kommande";
	grid-template-columns: 12fr 1fr 3fr;
	grid-template-rows: 4fr 2fr 8fr 1fr;
	overflow: hidden;
	> .notizen { grid-area: notizen; }
	> .kommande { grid-area: kommande; }
	> .informationen { grid-area: informationen; }
	> .arbeitsumgebung { grid-area: arbeitsumgebung; }
	> .applikationen { grid-area: applikationen; }
	> .boxen { grid-area: boxen; }
	> .zeuge { grid-area: zeuge; }
}
.aufsteller {
	> div {
		box-sizing: border-box;
		padding: 1rem;
		:global(> div) {
			padding: .8rem;
			background-color: #333;
			border: 2px solid gray;
			border-radius: 1rem;
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
