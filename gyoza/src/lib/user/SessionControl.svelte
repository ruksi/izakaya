<script lang="ts">
    import {revokeSessionMutation, type Session} from "$lib/backend";
    import {useQueryClient} from "@tanstack/svelte-query";

    const revoke = revokeSessionMutation(useQueryClient());
    export let session: Session;
    let isRevoking = false;

    function confirm() {
        $revoke.mutate({access_token_prefix: session.access_token_prefix});
    }

    $: if ($revoke.isSuccess) {
        isRevoking = false;
        $revoke.reset();
    }
</script>

<div class="flex">
    <main class="basis-96">
        <div>
            <span class="text-neutral-100">Access Token:</span>
            <span class="text-amber-200">
                {session.access_token_prefix}&hellip;
            </span>
        </div>
        <div>
            <span class="text-neutral-100">Last Use:</span>
            {session.used_at ? session.used_at : "Never"}
        </div>
    </main>
    <aside class="flex-1">
        {#if isRevoking}
            <form on:submit|preventDefault={confirm}>
                <span class="mr-2 select-none">Are you sure?</span>
                <button class="btn" on:click|preventDefault={() => isRevoking = false}>Cancel</button>
                <button class="btn btn-red" type="submit">Revoke</button>
            </form>
        {:else}
            <button class="btn btn-red" on:click|preventDefault={() => isRevoking = true}>Revoke</button>
        {/if}
    </aside>
</div>
