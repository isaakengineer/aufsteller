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
