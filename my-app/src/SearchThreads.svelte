<svelte:options immutable/>

<script lang="ts">
    import {tick} from 'svelte';

    /** @type {import('./$types').PageData} */

        // is populated via root +page.js on page load
    export let all_threads: {
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
        }[] = [];

    let search: string = '';
    let filteredThreads: {
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
    }[] = [];

    // prettier-ignore
    const filter = async () => {
        await tick();
        filteredThreads = search ? all_threads.filter((thread) => {
            return thread.site_process_thread_port.includes(search.toLowerCase());
        }) : [];
    };

    // for storing the setTimeout IDs
    const ids: number[] = [];

    // prettier-ignore
    async function handleUpdate(e: InputEvent & { target: HTMLInputElement; }) {
        // don't update immediately when user is deleting input by setting a delay
        if (e.inputType === 'deleteContentBackward' && e.target.value !== '') {
            // clear any existing scheduled timeouts first
            // clearTimeout doesn't panic for invalid IDs
            ids.forEach((id) => clearTimeout(id));
            ids.length = 0;
            ids.push(setTimeout(filter, 250));
        } else {
            await filter();
        }
    }
</script>

<div class="SearchThreads">
    <label for="searchbox"/>
    <input
            autofocus
            type="search"
            autocomplete="off"
            placeholder="Search for a Thread, IP/DNS, Port"
            id="searchbox"
            style="text-align: center"
            bind:value={search}
            on:input={handleUpdate}
    />

    <br/>
    <br/>

    <table id="live_thread_list">
        <tr>
            <th>IP</th>
            <th>Port</th>
            <th>Site</th>
            <th>Process</th>
            <th>Thread</th>
        </tr>
        {#each filteredThreads as thread}
            <tr>
                <td>{thread.ip}</td>
                <td>{thread.port}</td>
                <td>{thread.site}</td>
                <td>{thread.process}</td>
                <td><a href={thread.link}>{thread.thread}</a></td>
            </tr>
        {/each}
    </table>
</div>

<style>
    #live_thread_list th {
        border: 1px solid black;
        border-collapse: collapse;
        color: navajowhite;
    }

    #live_thread_list {
        width: 100%;
        min-width: max-content;
        color: navajowhite;
    }

    #searchbox {
        min-width: 50%;
    }

    #searchbox::placeholder {
        color: gold;
        font-size: medium;
    }

    .SearchThreads {
        width: 50%;
    }
</style>
