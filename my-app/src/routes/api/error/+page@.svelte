<script>
    //import { page } from '$app/stores';
    import {goto} from "$app/navigation";
</script>

<div id="error">
    <h2>The Thread Must Have an Inbound and/or Outbound SMAT Configured and Enabled</h2>
    <br>
    <br>
    <button on:click={() => goto('/')}>Click here to go back to the home page</button>

    <!--{#if $page?.error}
        <div class="mt-4 p-4 border-y-2">
            {#if $page?.status}
                <p>Page status: {$page?.status}</p>
            {/if}
            {#if $page?.error?.message}
                <p>Error message: {$page?.error?.message}</p>
            {/if}
        </div>
    {/if}-->
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
    #error {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        flex: 0.6;
    }

    h1 {
        width: 100%;
    }

    #error {
        max-width: 100%;
        min-height: 100%;
        margin: 0 auto;
        padding: 2rem;
        text-align: center;
        row-gap: 1rem;
    }
</style>
