import { error } from '@sveltejs/kit';

/** @type {import('./$types').PageLoad} */
export const csr = false;

export function load() {
    //return error(400, 'The Thread Must Have an Inbound and/or Outbound SMAT Configured and Enabled!');
}


