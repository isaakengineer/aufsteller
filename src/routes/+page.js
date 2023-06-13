/** @type {import('./$types').PageLoad} */

import { invoke } from "@tauri-apps/api/tauri";

export async function load({ params }) {

	let projects = await invoke("projects_list", {});

  return {
    projects: projects,
  };
}