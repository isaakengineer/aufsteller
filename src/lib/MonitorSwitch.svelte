<script>
	import { invoke } from '@tauri-apps/api/core';

	let password = "";
	async function switch_a() {
		invoke("switch_display_input", {
			id: 1,
			password: password,
		});
	}
	async function switch_b() {
		invoke("switch_display_input", {
			id: 2,
			password: password,
		});
	}
</script>

<div class="zeug">
<header>Monitor Switch</header>
	<div class="container">
			<button on:click={switch_a}>HDMI 1</button>
			<button on:click={switch_b}>HDML 2</button>
	</div>
	<div class="container">
		<input type="password" id="root" name="root"
			placeholder="Root Password"
			bind:value="{password}" />
	</div>
	<div class="hinweis">
		<p>Wird durch <code>ddcutil</code> fähigsein das Monitor zu wechseln.</p>
	</div>
</div>

<style lang="scss">
.zeug {
	border-color: pink !important;
	color: pink;
	> header {
		color: pink;
		font-weight: 800;
	}
	> .container {
		display: flex;
		justify-content: space-between;
	}
	button {
		border-color: deeppink;
		background-color: pink !important;
		background: none;
	}
}
.button-group {
	display: flex;
	justify-content: space-between;
}
.container {
	margin-top: 1rem;
	display: flex;
	gap: 1rem;
	vertical-align: baseline;
	input {
		flex-grow: 1;
	}
	button {
		padding: .2rem .8rem;
		font-size: .8rem;
		display: block;
		height: fit-content;
	}
	.label {
		padding: .4rem 0px;
		display: block;
	}
}
.hinweis {
	font-size: .8rem;
}
</style>
