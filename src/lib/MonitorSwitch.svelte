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
		<div class="label">
			<label for="monitor">Input</label>
		</div>
		<div class="button-group">
			<button on:click={switch_a}>HDMI 1</button>
			<button on:click={switch_b}>HDML 2</button>
		</div>
	</div>
	<div class="container">
		<div class="label">
			<label for="root">Root Password</label>
		</div>
		<input type="password" id="root" name="root"
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
}
.container {
	margin-top: 1rem;
	display: flex;
	gap: 1rem;
	vertical-align: baseline;
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
</style>
