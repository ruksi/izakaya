<script lang="ts">
    import {newSessionMutation} from "$lib/backend.js";
    import {useQueryClient} from "@tanstack/svelte-query";

    const newSession = newSessionMutation(useQueryClient());
    let isCreating = false;
    let password = "";

    function confirm() {
        $newSession.mutate({password});
    }

    $: if ($newSession.isSuccess) {
        password = "";
        isCreating = false;
        $newSession.reset();
    }
</script>

<div>
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
</div>
