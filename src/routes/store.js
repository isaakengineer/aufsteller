import { writable } from 'svelte/store';

const Ausstattung = writable({
	album: true,
	notizen: false,
	meldungen: [],
})

export { Ausstattung };
