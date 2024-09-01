<script lang="ts">
    import {goto} from "$app/navigation";

    /** @type {import('./$types').PageData} */

    export let data: {
        search_params: {
            site: string,
            process: string,
            thread_name: string,
            search: string,
            infile: string,
            outfile: string
        }
    };
</script>

<div id="select_smat">
    <h1>Which SMAT?</h1>
    <button on:click={() => goto(`/api/search?site=${data.search_params.site}&process=${data.search_params.process}&thread_name=${data.search_params.thread_name}&search=99999999999999999999&smat_file=${data.search_params.infile}`)}>
        <b>Inbound Save:</b> {data["search_params"].infile}
    </button>
    <br>
    <br>
    <br>
    <br>
    <button on:click={() => goto(`/api/search?site=${data.search_params.site}&process=${data.search_params.process}&thread_name=${data.search_params.thread_name}&search=99999999999999999999&smat_file=${data.search_params.outfile}`)}>
        <b>Outbound Save: </b>{data["search_params"].outfile}
    </button>

</div>


<svelte:head>
    <title>Cloverleaf Message Search</title>
    <meta name="description" content="" />
    <style>
        root {
            background-color: #242424;
            color: rgba(255, 255, 255, 0.87);
        }
        body {
            background-color: #242424;
            color: rgba(255, 255, 255, 0.87);
        }
        input {
            color-scheme: light dark;
        }
    </style>
</svelte:head>


<style>
    #select_smat {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        flex: 0.6;
    }

    h1 {
        width: 100%;
    }

    #select_smat {
        max-width: 100%;
        min-height: 100%;
        margin: 0 auto;
        padding: 2rem;
        text-align: center;
        row-gap: 1rem;
    }
</style>
