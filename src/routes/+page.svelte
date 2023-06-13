<script>
  import Greet from "$lib/Greet.svelte";
  export let data;

  import { invoke } from "@tauri-apps/api/tauri";

  async function projectOpen(project) {
    console.log(project)
    let projects = await invoke("project_on", {name: project});
  }
  let apps = {
    applets: [
      { name: 'Doket', url: 'https://doket.h.v-g.xyz'}, 
      { name: 'Wondrous', url: 'http://wondrous.h.v-g.xyz'}, 
      { name: 'Todo', url: 'https://todo.h.v-g.xyz'}, 
      { name: 'Vocabulog', url: 'https://vl.h.v-g.xyz/store'}, 
    ],
    websites: [
      { name: 'Grammarly', url: 'https://app.grammarly.com/' },
      { name: 'Webster', url: 'https://www.merriam-webster.com/' },
      { name: 'Longman', url: 'https://www.ldoceonline.com/' },
    ],
    serbian: [
      { name: 'Google T', url: 'https://translate.google.com' },
      { name: 'Dict', url: 'https://dict.com/serbian-english' },
    ],
    german: [
      { name: 'Pons', url: 'https://pons.de' },
      { name: 'Duden', url: 'https://duden.de' },
    ],
  };
</script>

<div class="workspace-conductor-app essential">
  <main>
    <div class="container_apps">
      <header>
        <div>Apps</div>
      </header>
      {#each Object.entries(apps) as [name, pack] }
        <div class="list_app">
          <header>{name}</header>
          
          {#each pack as app}
            <div class="app" on:click={() => {
                invoke("app_start", {name: app.url});
            }
            }>{app.name}</div>
          {/each}
        </div>
      {/each}
    </div>
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
.container_apps {
  margin: 1rem;
  > header {
    font-size: 1.5rem;
  }
  > .list_app {
    padding: 1rem;
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
    > header {
      text-transform: capitalize;
      font-weight: 800;
    }
    > .app {
      padding: .5rem;
      border: 2px solid gray;
      cursor: pointer;
      &:hover {
        border: 2px solid black;
      }
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