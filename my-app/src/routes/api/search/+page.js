/** @type {import('./$types').PageLoad} */
export function load({ params, route, url }) {
	let site = url.searchParams.get('site');
	let process = url.searchParams.get('process');
	let thread_name = url.searchParams.get('thread_name');
	let search = url.searchParams.get('search');
	let smat_file = url.searchParams.get('smat_file');


	return {
		search_params: {
			site: site,
			process: process,
			thread_name: thread_name,
			search: search,
			smat_file: smat_file
		}
	};
}
