<script lang="ts">
    import {logOutMutation, verifyQuery} from "$lib/backend";
    import {useQueryClient} from "@tanstack/svelte-query";

    const verify = verifyQuery();
    const logOut = logOutMutation(useQueryClient());

    function handleLogOut(e: MouseEvent) {
        e.preventDefault();
        $logOut.mutate();
    }

    $: if ($logOut.isSuccess) {
        window.location.href = "/";
    }
</script>

<header>
    <nav>
        <a href="/">🏠️</a>
        {#if $verify?.data?.is_authenticated}
            <a href="/settings">Settings</a>
            <a href="/log-out" on:click={handleLogOut}>Log Out</a>
        {:else}
            <a href="/log-in">Log In</a>
            <a href="/sign-up">Sign Up</a>
        {/if}
    </nav>
</header>

<style lang="postcss">
    header {
        @apply bg-zinc-850 border-b border-zinc-700;
        @apply text-[#88889b] text-sm text-nowrap;
    }
    nav {
        @apply flex gap-2 mx-auto max-w-7xl p-4;
    }
    a {
        @apply border border-[#88889b] rounded py-1 px-1.5 text-[#88889b];
        @apply hover:border-white hover:text-white;
    }
    a:first-of-type {
        @apply mr-auto;
    }
</style>
