<script lang="ts">
    import {newSessionMutation} from "$lib/backend.js";
    import {useQueryClient} from "@tanstack/svelte-query";

    const create = newSessionMutation(useQueryClient());
    let isCreating = false;
    let password = "";

    function confirm() {
        $create.mutate({password});
    }

    $: if ($create.isSuccess) {
        console.log("todo render:", $create.data);
        password = "";
        isCreating = false;
        $create.reset();
    }
</script>

<div class="flex">
    <main class="basis-96">
        todo placeholder
    </main>
    <aside>
        {#if isCreating}
            <form on:submit|preventDefault={confirm}>
                <!-- svelte-ignore a11y-autofocus -->
                <input
                    type="password"
                    placeholder="Re-type your password..."
                    bind:value={password}
                    autofocus
                >
                <button class="btn btn-purple" type="submit">Confirm</button>
            </form>
        {:else}
            <button class="btn" on:click|preventDefault={() => isCreating = true}>
                Create API token
            </button>
        {/if}
    </aside>
</div>
