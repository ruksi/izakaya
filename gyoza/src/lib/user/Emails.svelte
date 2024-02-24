<script lang="ts">
    import {emailsQuery} from "$lib/backend";

    const query = emailsQuery();
    $: emails = $query?.data ?? [];
</script>

{#if emails.length }
    <div class="border border-zinc-700 rounded-lg max-w-96">
        {#each emails as email}
            <div class="overflow-x-clip px-4 py-2 border-b border-zinc-700">
                <div>
                    {email.email}
                    {#if email.is_primary}
                        – <span class="text-green-400">Primary</span>
                    {/if}
                </div>
                {#if email.is_primary}
                    <div class="text-sm text-neutral-400/80 leading-tight">
                        This email is used for password recovery.
                    </div>
                {/if}
            </div>
        {/each}
    </div>
{/if}
