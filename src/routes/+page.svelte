<script>
  import Greet from "$lib/Greet.svelte";
  export let data;

  import { invoke } from '@tauri-apps/api/core';

  async function projectOpen(project) {
    console.log(project)
    let projects = await invoke("project_on", {name: project});
  }
  import WebAppBox from "$lib/WebAppBox.svelte";
  import MonitorSwitch from "$lib/MonitorSwitch.svelte";
  import TaboretBox from "$lib/TaboretBox.svelte";
  import DirectoryBox from "$lib/DirectoryBox.svelte";
</script>

<div class="workspace-conductor-app essential">
  <main class="dashboards">
    <section>
      <MonitorSwitch />
    </section>
    <section>
      <TaboretBox />
    </section>
    <section>
      <DirectoryBox />
    </section>
    <section>
      <WebAppBox />
    </section>
  </main>
  <div class="drawer dashboard-container dashboard view-dashboard">
    <div class="control-tabs">
      {#each data.projects as project}
          <div class="tab">
            <span>{project}</span>
          <button class="action" on:click={() => {
            console.log(project)
            projectOpen(project)}}
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-power" width="44" height="44" viewBox="0 0 24 24" stroke-width="1.5" stroke="#2c3e50" fill="none" stroke-linecap="round" stroke-linejoin="round">
              <path stroke="none" d="M0 0h24v24H0z" fill="none"/>
              <path d="M7 6a7.75 7.75 0 1 0 10 0" />
              <line x1="12" y1="4" x2="12" y2="12" />
            </svg>
          </button>
        </div>
      {/each}
    </div>
  </div>
</div>
<div class="row">
</div>

<style lang="scss">
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
