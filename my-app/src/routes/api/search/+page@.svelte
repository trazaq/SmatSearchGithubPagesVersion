<script lang="ts">
    //import HL7Dictionary from "hl7-dictionary";
    //let PUBLIC_PORT = "17600"
    import {prettyPrintJson} from "pretty-print-json";

    /** @type {import('./$types').PageData} */

    export let data: {
        search_params: {
            site: string,
            process: string,
            thread_name: string,
            search: string,
            smat_file: string
        },
        messages: string[]
    };

    let site = data.search_params.site ?? "";
    let process = data.search_params.process ?? "";
    let thread_name = data.search_params.thread_name ?? "";
    // bound to the form
    let date1 = new Date().toLocaleDateString("sv-SE"); // returns as YYYY-MM-DD
    let date2 = new Date().toLocaleDateString("sv-SE"); // returns as YYYY-MM-DD
    let search = "";
    let msg_limit = 20;
    let case_sensitive = true;

    let returned_messages: { type: string; data: string }[] = [];
    let smat_dbs: string[] = [];
    let positive_smats: string[] = [];
    let total_msgs_searched = 0;
    let search_limit_reached = false;
    let errors: string[] = [];

    let disabled = false;

    // prettier-ignore
    async function handleSubmit() {
        disabled = true; // disable the submit button to avoid repeated requests by the user...

        try {
            returned_messages = search_db().messages.map((msg: string, i: number, array) => {
                // if this is a JSON message, pretty-print it
                let json = formatJson(msg);
                if (json) {
                    return {type: 'json', data: json}
                } else {
                    const segs = msg.includes("\n") ? msg.split("\n") : msg.split("\r");
                    segs.forEach(function (seg, i) {
                        if (seg) {
                            const fields = seg.split("|");
                            if (fields.length > 0) {
                                //bold the seg names
                                const segname = fields[0].trim();
                                let isMSH = segname === "MSH";
                                fields[0] = "<strong>" + segname + "</strong>";

                                //bold the message type
                                if (isMSH && fields.length >= 8) fields[8] = "<strong>" + fields[8] + "</strong>";

                                for (let i = 0; i < fields.length; ++i) {
                                    //Only add a span if there is data in the field
                                    if (fields[i]) {
                                        let desc = "";
                                        try {
                                            const index = isMSH ? i : i - 1;
                                            //desc = HL7Dictionary.definitions["2.7"].segments[segname].fields[index].desc;
                                        } catch (e) {/* empty */
                                        }
                                        //console.log(desc)
                                        //incr i by 1 if dealing with the MSH segment
                                        fields[i] = `<span class="${segname}-${isMSH ? i + 1 : i}" title="${segname}-${isMSH ? i + 1 : i}: ${desc}">${fields[i]}</span>`;
                                    }
                                }
                            }
                            segs[i] = fields.join("|");
                        }
                    });
                    //array[i] = segs.join("<br>");
                    return {type: 'hl7', data: segs.join("<br>")}
                }
            });
            total_msgs_searched = search_db().total_msgs;
            smat_dbs = search_db().smatdbs;
            positive_smats = search_db().positive_smats;
            search_limit_reached = search_db().search_limit_reached ?? false;
            try {
                errors = search_db().errors;
            } catch (e) {
                errors = [];
            }
        } catch (e) {
            console.log(e);
            returned_messages.length = 0;
            returned_messages.push({type: 'error', data: `${e}`});
        }

        disabled = false;
    }

    // prettier-ignore
    function formatJson(json: string): string | false {
        try {
            json = JSON.parse(json);
            return prettyPrintJson.toHtml(json);
        } catch (e) {
            return false
        }
    }

    function search_db(): {
        messages: string[];
        total_msgs: number;
        smatdbs: string[];
        positive_smats: string[];
        search_limit_reached: boolean;
        errors: string[];
    } {
        return {
            messages: data.messages,
            total_msgs: data.messages.length,
            smatdbs: ['here is the smatdb.smat'],
            positive_smats: ['here is the smatdb.smat'],
            search_limit_reached: false,
            errors: []
        }
    }
</script>

<svelte:head>
    <style>
        root {
            background-color: #e5e5e5;
            color: black;
        }

        body {
            background-color: #e5e5e5;
            color: black;
        }
    </style>
</svelte:head>

<div class="search">
    <h2>View Messages for <strong><em>{thread_name}</em></strong></h2>

    <form method="GET" on:submit|preventDefault={handleSubmit}>
        <input id="datepicker1" type="date" name="date" bind:value={date1}/> - <input id="datepicker2" type="date"
                                                                                      name="date" bind:value={date2}/>
        <br/>
        <br/>
        <label>
            <input type="number" name="search" bind:value={msg_limit}/> (Message limit)
        </label>
        <br/>
        <br/>
        <label>
            <input type="search" name="search" bind:value={search}/> (If two or more
            values must be present in the message, use '&&') (Example: "A04&&12345")
        </label>
        <br>
        <label>
            <input type="checkbox" id="case_sensitive" name="case_sensitive" bind:checked={case_sensitive}>
            case sensitive search?
        </label>
        <br/>
        <br/>
        <button type="submit" {disabled}>Submit</button>
    </form>

    {#each smat_dbs as file}
        {#if positive_smats.includes(file)}
            <p><em><strong>{file}</strong></em></p>
        {:else}
            <p>{file}</p>
        {/if}
    {/each}

    {#if errors}
        {#each errors as err}
            <strong>{err}</strong>
            <br/>
        {/each}
    {/if}

    <!--  search.length used for checking if the user entered a query so we use the appropriate verbiage-->
    {#if returned_messages.length > 0}
        {#if search.length > 0}
            <p>
                <strong
                >Found {returned_messages.length} out of: {total_msgs_searched}</strong
                >
            </p>
        {:else}
            <p>
                <strong
                >Retrieved first {returned_messages.length} out of: {total_msgs_searched}</strong
                >
            </p>
        {/if}
    {/if}

    <div>
        {#each returned_messages as message}
            {#if message.type === "hl7" || message.type === 'error'}
                <p>{@html message.data}</p>
            {:else}
                <pre class="json-container">{@html message.data}</pre>
            {/if}
        {/each}
    </div>
</div>

<style>
    .search {
        margin: 2em;
    }

    .json-container {
        /* make it the same as the page's color */
        background-color: inherit;
        padding-left: 5px;

        border-style: outset;
        border-left-width: thin;
        border-top: none;
        border-bottom: none;
        border-right: none;
    }
</style>
