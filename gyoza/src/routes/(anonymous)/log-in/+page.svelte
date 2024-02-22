<script lang="ts">
    import Alert from "$lib/Alert.svelte";
    import {logInMutation} from "$lib/backend";
    import {useQueryClient} from "@tanstack/svelte-query";
    import {slide} from "svelte/transition";

    const logIn = logInMutation(useQueryClient());

    let username_or_email = "";
    let password = "";

    function handleSubmit(e: SubmitEvent) {
        e.preventDefault();
        e.stopPropagation();
        $logIn.mutate({username_or_email, password});
    }

    $: if ($logIn.isSuccess) {
        if ($logIn?.data?.status == "ok") {
            window.location.href = "/";
        }
    }
</script>

<svelte:head>
    <title>Log In</title>
    <meta name="description" content="Log in to your Izakaya account" />
</svelte:head>

<!-- svelte-ignore a11y-autofocus -->
<main class="flex flex-col max-w-sm mx-auto mt-10 px-3">
    {#if $logIn.error}
        <div class="pb-4" transition:slide>
            <Alert error={$logIn.error} color="red" head="Login Failed" />
        </div>
    {/if}
    <form class="card" on:submit={handleSubmit}>
        <div class="card-head py-3 px-4 text-xl">
            <h1>Log In</h1>
        </div>
        <div class="card-body py-5 px-6 flex flex-col gap-4">
            <label>
                Username <span class="text-sm text-zinc-400">or Email</span>
                <input name="username_or_email" type="text" bind:value={username_or_email} autofocus />
            </label>
            <label>
                Password
                <input name="password" type="password" bind:value={password} />
            </label>
        </div>
        <div class="card-foot py-3 px-4 mt-3 flex">
            <button class="ms-auto btn btn-purple">Log in</button>
        </div>
    </form>
    <div class="pt-12 text-center text-gray-400">
        New to Izakaya?
        <a href="/sign-up" class="text-purple-400"> Create an account </a>
    </div>
</main>

<style lang="postcss">
    input[type] {
        @apply py-1 px-2 rounded-lg w-full;
    }
</style>
