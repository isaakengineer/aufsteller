<script>
	import { invoke, convertFileSrc } from "@tauri-apps/api/core";
	import { onMount } from "svelte";
	import { fade, slide, blur } from "svelte/transition";

	import markdownit from "markdown-it";
	import { Ausstattung } from "./store";

	export let data;

	const md = markdownit();

	let items = [];

	// async function projectOpen(project) {
	// 	console.log(project)
	// 	let projects = await invoke("project_on", {name: project});
	// }

	let profilen = [];
	let profile = "default";

	let webseiten = [];
	let websuchen = [];
	let taboreten = [];
	let dirs = [];

	let bildern = [];
	let bildernIndex = 0;
	let notizen = [];
	let notizenIndex = 0;

	onMount(async () => {
		bildern = await invoke("album_init");
		notizen = await invoke("notizen_init");
		profilen = await invoke("profile_lesen");
		await init();
	});
	console.log("these are items");
	console.log(items);

	const initError = (e) => {
		Ausstattung.update((a) => {
			a.meldungen.push(e);
			return a;
		});
		console.warn(e);
		setTimeout(() => {
			Ausstattung.update((a) => {
				a.meldungen.pop();
				return a;
			});
		}, 5000);

		return [];
	};
	const init = async (profile) => {
		switch (profile) {
			case "default":
				bildern = await invoke("album_init");
				notizen = await invoke("notizen_init");
				webseiten = await invoke("csv_lesen", { fach: "Web" }).catch(
					initError,
				);
				websuchen = await invoke("csv_lesen", { fach: "Suche" }).catch(
					initError,
				);
				taboreten = await invoke("csv_lesen", {
					fach: "Taboret",
				}).catch(initError);
				dirs = await invoke("csv_lesen", { fach: "Bookmark" }).catch(
					initError,
				);
				break;
			default:
				bildern = await invoke("album_init", { profile: profile });
				notizen = await invoke("notizen_init", { profile: profile });
				webseiten = await invoke("csv_lesen", {
					fach: "Web",
					profile: profile,
				}).catch(initError);
				websuchen = await invoke("csv_lesen", {
					fach: "Suche",
					profile: profile,
				}).catch(initError);
				taboreten = await invoke("csv_lesen", {
					fach: "Taboret",
					profile: profile,
				}).catch(initError);
				dirs = await invoke("csv_lesen", {
					fach: "Bookmark",
					profile: profile,
				}).catch(initError);
		}
	};

	const profileWaechseln = async (p) => {
		profile = p;
		init(p);
	};

	import Webseite from "$lib/Webseite.svelte";
	import Websuche from "$lib/Websuche.svelte";
	import MonitorSwitch from "$lib/MonitorSwitch.svelte";
	import TaboretBox from "$lib/TaboretBox.svelte";
	import DirectoryBox from "$lib/DirectoryBox.svelte";
</script>

<div class="aufsteller">
	<div class="notizen">
		{#if $Ausstattung.album}
			<!-- <div class="album" transition:slide|global> -->
			<div class="album">
				{#if bildern.length > 0}
					<div class="bilderrahmen">
						{#key bildernIndex}
							<!-- <img in:blur|global={{ duration: 800 }} out:fade|global src={convertFileSrc(bildern[bildernIndex])} /> -->
							<img src={convertFileSrc(bildern[bildernIndex])} />
						{/key}
					</div>
				{/if}
				<div class="albumkontrollen">
					<button
						on:click={() => {
							bildernIndex =
								(bildernIndex == 0
									? bildern.length
									: bildernIndex) - 1;
						}}>Vorherrige</button
					>
					<button
						on:click={() => {
							$Ausstattung.notizen = true;
							$Ausstattung.album = false;
						}}>Notizen</button
					>
					<button
						on:click={() => {
							bildernIndex = (bildernIndex + 1) % bildern.length;
						}}>Nächste</button
					>
				</div>
			</div>
		{:else if $Ausstattung.notizen}
			<!-- <div transition:slide|global> -->
			<div>
				<div class="notizenkontrollen">
					<button
						on:click={() => {
							$Ausstattung.notizen = false;
							$Ausstattung.album = true;
						}}>Album</button
					>
					<button on:click={() => {}}>Open</button>
				</div>
				<div class="notizrahmen">
					{#each notizen as notiz, i}
						<div class="notiz">
							<header
								on:click={() => {
									notizenIndex = i;
								}}
							>
								{notiz.name}
							</header>
							{#if notizenIndex == i}
								<pre class="inhalt">{@html md.render(
										notiz.inhalt,
									)}</pre>
							{/if}
						</div>
					{/each}
				</div>
			</div>
		{/if}
	</div>
	<div class="kommande">
		{#if $Ausstattung.meldungen.length > 0}
			{#each $Ausstattung.meldungen as m}
				<div class="meldung">
					<div class="nachricht">
						{m}
						<button
							class="kontrolle"
							on:click={() => {
								Ausstattung.update((a) => {
									a.meldungen = a.meldungen.slice(0, -1);
									console.log("we removed one!", a);
									return a;
								});
							}}>X</button
						>
					</div>
				</div>
			{/each}
		{/if}
	</div>
	<div class="informationen"></div>
	<div class="arbeitsumgebung">
		<nav class="profilen">
			<div on:click={() => profileWaechseln("default")}>Default</div>
			{#each profilen as p}
				<div on:click={profileWaechseln(p.name)}>{p.name}</div>
			{/each}
		</nav>
	</div>
	<div class="applikationen"></div>
	<div class="zeuge">
		<div class="titel">{profile}</div>
		<MonitorSwitch />
	</div>
	<div id="masonry-grid" class="boxen">
		<TaboretBox listen={taboreten} />
		<DirectoryBox listen={dirs} />
		<Webseite links={webseiten} />
		<Websuche links={websuchen} />
	</div>
</div>

<style lang="scss">
	.zeuge {
		display: flex;
		flex-direction: row;
		> .titel {
			margin: 0.42rem 2rem 0.42rem 0px;
			width: 3.8rem;
			box-sizing: border-box;
			/* height: 10rem; */
			box-sizing: border-box;
			padding: 1.2rem 1rem;
			/* margin-top: 2rem; */
			height: fit-content;
			font-size: 1.8rem;
			/* transform: rotate(90deg); */
			color: #fff;
			border-radius: 0px;
			border: none;
			writing-mode: vertical-lr;
			text-transform: capitalize;
		}
	}
	nav.profilen {
		display: flex;
		flex-direction: column;
		/* gap: 0.5rem; */
		padding: 0.5rem 0px 0.5rem 0.5rem;
		div {
			padding: 0.5rem;
			/* width: fit-content; */
			font-size: 1.2rem;
			background-color: rgba(255, 255, 255, 0.7);
			box-shadow: 0px 0px 0.2rem rgba(0, 0, 0, 0.3);
			cursor: pointer;
			&:hover {
				background-color: rgba(255, 255, 255, 0.9);
				box-shadow: 0px 0px 0.2rem rgba(0, 0, 0, 0.5);
			}
		}
	}
	.meldung {
		border-radius: 0 !important;
		border: none;
		box-shadow: none;
		background-color: transparent;
		display: flex;
		justify-content: space-between;
		background-color: #ccffff;
		margin: 0px 0.5rem;
		padding: 0.5rem;
		> .nachricht {
			/* padding: 0.5rem; */
			margin: 0.5rem;
			> button {
				display: inline-block;
				background: transparent;
				border: none;
			}
		}
	}
	#masonry-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(12rem, 1fr));
		grid-gap: 1rem;
		grid-auto-rows: minmax(4rem, auto);
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
			&.span-3 {
				grid-column-end: span 3;
				grid-row-end: span 4;
			}
		}
	}
	:global(#masonry-grid > .grid-item.span-2) {
		grid-column-end: span 2;
		grid-row-end: span 2;
	}
	button {
		padding: 0.3rem 0.8rem;
	}
	.notizrahmen {
		background-color: transparent !important;
		padding: 0px !important;
		border: none !important;
		display: flex;
		flex-direction: column;
		gap: 0.2rem;
		> .notiz {
			background-color: lightyellow;
			box-shadow: 0px 0px 0.25rem rgba(0, 0, 0, 0.5);
			> header {
				padding: 0.1rem 0.5rem;
				border-bottom: 1px dashed yellow;
				font-size: 0.9rem;
				&:hover {
					cursor: pointer;
				}
			}
			> .inhalt {
				padding: 0.5rem 1rem;
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
		/* overflow-y: scroll; */
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
			border-radius: 0.5rem;
			box-shadow: 0px 0px 0.25rem rgba(0, 0, 0, 0.5);
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
		padding: 0.8rem;
		box-sizing: border-box;
		/* gap: 0.8rem; */
		grid-template-areas:
			"boxen 			arbeitsumgebung zeuge"
			"boxen 			arbeitsumgebung applikationen"
			"boxen 			arbeitsumgebung notizen"
			"informationen 	informationen 	notizen"
			"informationen 	informationen 	kommande";
		grid-template-columns: 13fr 10rem 5fr;
		grid-template-rows: 1fr 5rem 1fr 4fr 5rem;
		overflow: hidden;
		> .notizen {
			grid-area: notizen;
		}
		> .kommande {
			grid-area: kommande;
		}
		> .informationen {
			grid-area: informationen;
		}
		> .arbeitsumgebung {
			grid-area: arbeitsumgebung;
		}
		> .applikationen {
			grid-area: applikationen;
		}
		> .boxen {
			grid-area: boxen;
		}
		> .zeuge {
			grid-area: zeuge;
		}
		> div {
		}
	}
	.aufsteller {
		> div {
			/* box-sizing: border-box;
		padding: 1rem; */

			:global(> div) {
				padding: 0.8rem;
				background-color: #333;
				border: 2px solid gray;
				border-radius: 1rem;
				box-shadow: 0px 0px 0.25rem rgba(0, 0, 0, 0.5);
			}
		}
	}
	.essential {
		background-color: transparent !important;
	}

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
	.kommande {
		overflow-y: auto;
	}
</style>
