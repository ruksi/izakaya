<script lang="ts">
    import {logOutMutation, verifyQuery} from "$lib/backend";
    import {useQueryClient} from "@tanstack/svelte-query";

    const verify = verifyQuery();
    const logOut = logOutMutation(useQueryClient())

    function logOutClick(e: MouseEvent) {
        e.preventDefault();
        $logOut.mutate();
    }
</script>

<nav>
    <a href="/">🏠️</a>
    {#if $verify?.data?.is_authenticated}
        <a href="/log-out" on:click={logOutClick}>Log Out</a>
    {:else}
        <a href="/log-in">Log In</a>
        <a href="/sign-up">Sign Up</a>
    {/if}
</nav>
