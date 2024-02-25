<script lang="ts">
    import {type Session, sessionsQuery} from "$lib/backend";

    const sortByToken = (a: Session, b: Session) => {
        if (a.access_token_prefix < b.access_token_prefix) {
            return -1;
        }
        if (a.access_token_prefix > b.access_token_prefix) {
            return 1;
        }
        return 0;
    };

    const query = sessionsQuery();
    $: sessions = $query?.data?.sort(sortByToken) ?? [];
</script>

<div class="flex flex-col gap-3">
    {#each sessions as session (session.access_token_prefix)}
        <div>
            <div>
                <span class="text-neutral-100">
                    Access Token:
                </span>
                <span class="text-amber-200">
                    {session.access_token_prefix}&hellip;
                </span>
            </div>
            <div>
                <span class="text-neutral-100">
                    Last Use:
                </span>
                {session.used_at ? session.used_at : "Never"}
            </div>
        </div>
    {/each}
</div>
