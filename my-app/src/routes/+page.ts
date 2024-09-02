import type {PageLoad} from './$types';

export const load: PageLoad = async ({fetch, url, route, params}) => {
    let all_threads: ({
        site: string,
        process: string,
        thread: string,
        ip: string,
        port: string,
        insave: string,
        infile: string,
        outsave: string,
        outfile: string,
        link: string,
        site_process_thread_port: string
    })[];

    all_threads = sample_data().map(val => {

        const link = generateLinke(val);
        return {
            ...val,
            link: link,
            site_process_thread_port: `${val.site} ${val.process} ${val.thread} ${val.ip} ${val.port}`.toLowerCase()
        }

    })

    return {all_threads: all_threads};
};

function generateLinke(val: {
    site: string,
    process: string,
    thread: string,
    ip: string,
    port: string,
    insave: string,
    infile: string,
    outsave: string,
    outfile: string,
}): string {

    const date = new Date().toLocaleDateString('sv-SE'); // returns as YYYY-mm-dd
    let link = "";

    // redirect to select_smat
    if (val.insave == "1" && val.outsave == "1") {
        link = `/SmatSearchGithubPagesVersion.github.io/api/select_smat?site=${val.site}&process=${val.process}&thread_name=${val.thread}&search=99999999999999999999&infile=${val.infile}&outfile=${val.outfile}&date=${date}`;
    }

    if (val.insave == "1" && val.outsave == "0") {
        link = `/SmatSearchGithubPagesVersion.github.io/api/search?site=${val.site}&process=${val.process}&thread_name=${val.thread}&search=99999999999999999999&smat_file=${val.infile}&date=${date}`;
    }

    if (val.insave == "0" && val.outsave == "1") {
        link = `/SmatSearchGithubPagesVersion.github.io/api/search?site=${val.site}&process=${val.process}&thread_name=${val.thread}&search=99999999999999999999&smat_file=${val.outfile}&date=${date}`;
    }

    if (val.insave == "0" && val.outsave == "0") {
        link = `/SmatSearchGithubPagesVersion.github.io/api/error?site=${val.site}&process=${val.process}&thread_name=${val.thread}&search=99999999999999999999&smat_file=&date=${date}`;
    }

    if (val.insave == "" && val.outsave == "") {
        link = `/SmatSearchGithubPagesVersion.github.io/api/error?site=${val.site}&process=${val.process}&thread_name=${val.thread}&search=99999999999999999999&smat_file=&date=${date}`;
    }

    return link;
}


function sample_data(): {
    site: string,
    process: string,
    thread: string,
    ip: string,
    port: string,
    insave: string,
    infile: string,
    outsave: string,
    outfile: string,
}[] {
    return [
        {
            "site": "verity",
            "process": "verity",
            "thread": "testdb1",
            "ip": "localhost",
            "port": "8080",
            "insave": "1",
            "infile": "testdb1",
            "outsave": "0",
            "outfile": ""
        },
        {
            "site": "verity",
            "process": "verity",
            "thread": "no_smat_configured",
            "ip": "",
            "port": "",
            "insave": "0",
            "infile": "",
            "outsave": "0",
            "outfile": ""
        },
        {
            "site": "verity",
            "process": "verity",
            "thread": "both_smat_configured",
            "ip": "",
            "port": "",
            "insave": "1",
            "infile": "testdb1",
            "outsave": "1",
            "outfile": "testdb1"
        }];
}