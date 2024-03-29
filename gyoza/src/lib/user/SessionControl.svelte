<script lang="ts">
    import {revokeSessionMutation, type Session} from "$lib/backend";
    import {useQueryClient} from "@tanstack/svelte-query";
    import {formatDistance, parseISO} from "date-fns";

    const revoke = revokeSessionMutation(useQueryClient());
    export let session: Session;
    let isRevoking = false;

    function confirm() {
        $revoke.mutate({access_token_prefix: session.access_token_prefix});
    }

    $: usedAtHuman = session.used_at ? formatDistance(parseISO(session.used_at), new Date(), {addSuffix: true}) : "";

    $: if ($revoke.isSuccess) {
        isRevoking = false;
        $revoke.reset();
    }
</script>

<div class="flex flex-wrap">
    <main class="basis-80">
        <div>
            <span class="text-neutral-100">Access Token:</span>
            <span class="text-amber-200">
                {session.access_token_prefix}&hellip;
            </span>
        </div>
        <div>
            <span class="text-neutral-100">Last Used:</span>
            {#if session.used_at}
                <abbr title={session.used_at}>{usedAtHuman}</abbr>
            {:else}
                <span class="text-neutral-500">Never</span>
            {/if}
        </div>
    </main>
    <aside class="flex-1">
        {#if isRevoking}
            <form on:submit|preventDefault={confirm}>
                <span class="mr-2 select-none">Are you sure?</span>
                <button class="btn btn-sm btn-outline" on:click|preventDefault={() => isRevoking = false}>
                    Cancel
                </button>
                <button class="btn btn-sm btn-error" type="submit">Revoke</button>
            </form>
        {:else}
            <button class="btn btn-sm btn-error" on:click|preventDefault={() => isRevoking = true}>Revoke</button>
        {/if}
    </aside>
</div>
