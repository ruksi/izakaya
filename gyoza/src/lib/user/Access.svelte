<script lang="ts">
    import {type Session, sessionsQuery} from "$lib/backend";
    import NewSessionControl from "$lib/user/NewSessionControl.svelte";
    import SessionControl from "$lib/user/SessionControl.svelte";

    const sortByAccessToken = (a: Session, b: Session) => {
        if (a.access_token_prefix < b.access_token_prefix) {
            return -1;
        }
        if (a.access_token_prefix > b.access_token_prefix) {
            return 1;
        }
        return 0;
    };

    const query = sessionsQuery();
    $: sessions = $query?.data?.sort(sortByAccessToken) ?? [];
</script>

<div class="flex flex-col gap-3">
    {#each sessions as session (session.access_token_prefix)}
        <SessionControl {session} />
    {/each}
    <NewSessionControl />
</div>
