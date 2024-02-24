<script lang="ts">
    import {currentUserQuery} from "$lib/backend";

    const query = currentUserQuery();
    $: user = $query?.data ?? undefined;
</script>

<span class="text-neutral-100">Username:</span>
{#if user }
    {user.username}
{/if}
