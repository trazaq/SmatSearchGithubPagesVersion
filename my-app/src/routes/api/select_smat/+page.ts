/** @type {import('./$types').PageLoad} */

export const csr = false;
export function load({ params, route, url }) {
    let site = url.searchParams.get('site');
    let process = url.searchParams.get('process');
    let thread_name = url.searchParams.get('thread_name');
    let search = url.searchParams.get('search');
    let infile = url.searchParams.get('infile');
    let outfile = url.searchParams.get('outfile');


    return {
        search_params: {
            site: site,
            process: process,
            thread_name: thread_name,
            search: search,
            infile: infile,
            outfile: outfile
        }
    };
}
