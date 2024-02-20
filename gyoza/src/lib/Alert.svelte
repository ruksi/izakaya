<script lang="ts">
    export let head: string | undefined = undefined;
    export let body: string | undefined = undefined;
    export let error: Error | undefined = undefined;
    export let color: "gray" | "red" | "yellow" | "green" = "gray";

    $: _head = head ?? error?.data?.message ?? "";
    $: _body = body ?? (error?.status ? `${error.status} ` : "") + (error?.message ?? "Error");
</script>

<div class="alert {color}">
    <div class="head">
        {_head}
    </div>
    <div class="body">
        {_body}
    </div>
</div>

<style lang="postcss">
    .alert {
        @apply px-4 py-3 border rounded break-words;
    }
    .head {
        @apply font-bold;
    }

    .gray {
        @apply bg-gray-950 text-gray-300 border-gray-500;
    }
    .gray .body {
        @apply text-gray-400;
    }

    .red {
        @apply bg-red-990 text-red-300 border-red-900;
    }
    .red .body {
        @apply text-red-400;
    }

    .yellow {
        @apply bg-yellow-990 text-yellow-500 border-yellow-900;
    }
    .yellow .body {
        @apply text-yellow-600;
    }

    .green {
        @apply bg-green-990 text-green-500 border-green-900;
    }
    .green .body {
        @apply text-green-600;
    }
</style>
