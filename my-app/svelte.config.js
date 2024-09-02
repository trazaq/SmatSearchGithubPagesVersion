import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter({
			// default options are shown. On some platforms
			// these options are set automatically — see below
			pages: 'build',
			assets: 'build',
			fallback: 'app.html',
			precompress: false,
			strict: true
		}),
	},
	preprocess: [vitePreprocess({})],
	vitePlugin: {
		inspector: true
	}
};

export default config;
