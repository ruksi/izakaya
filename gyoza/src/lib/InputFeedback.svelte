<script lang="ts">
    import {isBackendError, type IssueDetailMap} from "$lib/errors";
    import {formatMessage} from "$lib/messages";
    import {slide} from "svelte/transition";

    export let field: string;
    export let error: Error | null = null;
    $: _error = isBackendError(error) ? error : null;

    let fieldIssues: IssueDetailMap;
    $: fieldIssues = (_error && _error.data && _error.data.issues && _error.data.issues[field]) ?? {};
</script>

{#each Object.values(fieldIssues) as issue}
    <div class="text-error text-sm pl-1 pt-0.5" transition:slide>
        {formatMessage(issue)}
    </div>
{/each}
