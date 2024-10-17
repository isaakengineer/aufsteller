import staticAdapter from "@sveltejs/adapter-static";
import preprocess from 'svelte-preprocess';
import { vitePreprocess } from '@sveltejs/kit/vite';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: [preprocess()],
  kit: {
    adapter: staticAdapter(),
  },
};

export default config;
