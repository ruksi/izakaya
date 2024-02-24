<script lang="ts">
    import Alert from "$lib/Alert.svelte";
    import {signUpMutation} from "$lib/backend";
    import {useQueryClient} from "@tanstack/svelte-query";
    import {slide} from "svelte/transition";

    const signUp = signUpMutation(useQueryClient());

    let email = "";
    let username = "";
    let password = "";

    function handleSubmit(e: SubmitEvent) {
        e.preventDefault();
        e.stopPropagation();
        $signUp.mutate({email, username, password});
    }

    $: if ($signUp.isSuccess) {
        if ($signUp?.data?.status == "ok") {
            window.location.href = "/";
        }
    }
</script>

<svelte:head>
    <title>Sign Up</title>
    <meta name="description" content="Sign up for a new account">
</svelte:head>

<!-- svelte-ignore a11y-autofocus -->
<main class="flex flex-col max-w-sm mx-auto mt-10 px-3">
    {#if $signUp.error}
        <div class="pb-4" transition:slide>
            <Alert error={$signUp.error} color="red" head="Signup Failed" />
        </div>
    {/if}
    <form class="card" on:submit={handleSubmit}>
        <div class="card-head py-3 px-4 text-xl">
            <h1>Sign Up</h1>
        </div>
        <div class="card-body py-5 px-6 flex flex-col gap-4">
            <label>
                Email
                <input name="email" type="text" bind:value={email} autofocus />
            </label>
            <label>
                Username
                <input name="username" type="text" bind:value={username} />
            </label>
            <label>
                Password
                <input name="password" type="password" bind:value={password} />
            </label>
        </div>
        <div class="card-foot py-3 px-4 mt-3 flex">
            <button class="ms-auto btn btn-purple">Sign up</button>
        </div>
    </form>
    <div class="pt-12 text-center text-gray-400">
        Already got an account?
        <a href="/log-in" class="text-purple-400"> Log in here </a>
    </div>
</main>
