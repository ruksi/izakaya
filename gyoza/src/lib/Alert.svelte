<script lang="ts">
    import FetchError from "$lib/FetchError";

    let className = "";
    export {className as class};
    export let head: string | undefined = undefined;
    export let body: string | undefined = undefined;
    export let error: Error | FetchError | undefined = undefined;

    $: _head = head ?? error?.data?.message ?? "";
    $: _body = body ?? (error?.status ? `${error.status} ` : "") + (error?.message ?? "Error");
</script>

<div role="alert" class="alert {className}">
    <div>
        <header>
            {_head}
        </header>
        <div class="body">
            {_body}
        </div>
    </div>
</div>

<style lang="postcss">
    /*.alert {*/
    /*    @apply px-4 py-3 border rounded break-words;*/
    /*}*/
    header {
        @apply font-bold;
    }
</style>
