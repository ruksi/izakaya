<script lang="ts">
    import {newSessionMutation} from "$lib/backend.js";
    import {useQueryClient} from "@tanstack/svelte-query";

    const create = newSessionMutation(useQueryClient());
    let isCreating = false;
    let password = "";
    let modal: HTMLDialogElement | null = null;

    function confirm() {
        $create.mutate({password});
    }

    $: if ($create.isSuccess) {
        password = "";
        isCreating = false;
        if (modal) {
            modal.showModal();
            // this probably adds the event listener multiple times,
            // but it's fine for now 🤷
            modal.addEventListener("close", () => {
                $create.reset();
            });
        }
    }
</script>

<div class="flex flex-wrap">
    <main class="basis-80">
        &nbsp;
    </main>
    <aside class="">
        {#if isCreating}
            <form class="join" on:submit|preventDefault={confirm}>
                <!-- svelte-ignore a11y-autofocus -->
                <input
                    class="input input-sm join-item w-full sm:w-auto border-r-0"
                    type="password"
                    placeholder="Re-type your password..."
                    bind:value={password}
                    autofocus
                >
                <button class="btn btn-sm btn-primary join-item" type="submit">Confirm</button>
            </form>
        {:else}
            <button class="btn btn-sm btn-outline" on:click|preventDefault={() => isCreating = true}>
                Create API token
            </button>
        {/if}
    </aside>
</div>

<dialog class="modal" bind:this={modal}>
    <div class="modal-box">
        <h3 class="font-bold text-lg">Your API Token is Ready!</h3>
        <p class="py-4">Copy the following token:</p>
        <p class="font-mono break-words text-amber-200">
            {$create.data?.access_token}
        </p>
        <p class="py-4">
            <span class="font-bold">Keep this token safe!</span>
            You won't be seeing it again.
        </p>
        <div class="modal-action">
            <form method="dialog">
                <button class="btn btn-sm btn-outline">Yes, I copied it</button>
            </form>
        </div>
    </div>
</dialog>
