<script lang="ts">
    import Alert from "$lib/Alert.svelte";
    import {signUpMutation} from "$lib/backend";
    import InputFeedback from "$lib/InputFeedback.svelte";
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
    {#if $signUp.error && $signUp.error.status !== 400}
        <div class="pb-4" transition:slide>
            <Alert class="alert-error" error={$signUp.error} head="Signup Failed" />
        </div>
    {/if}
    <form class="card" on:submit={handleSubmit}>
        <div class="card-head py-3 px-4 text-xl">
            <h1>Sign Up</h1>
        </div>
        <div class="py-5 px-6 flex flex-col gap-4">
            <label class="form-control">
                <div class="label">
                    Email
                </div>
                <input
                    class="input input-sm"
                    name="email"
                    type="text"
                    bind:value={email}
                    autofocus
                />
                <InputFeedback field="email" error={$signUp.error} />
            </label>
            <label class="form-control">
                <div class="label">
                    Username
                </div>
                <input
                    class="input input-sm"
                    name="username"
                    type="text"
                    bind:value={username}
                />
                <InputFeedback field="username" error={$signUp.error} />
            </label>
            <label class="form-control">
                <div class="label">
                    Password
                </div>
                <input
                    class="input input-sm"
                    name="password"
                    type="password"
                    bind:value={password}
                />
                <InputFeedback field="password" error={$signUp.error} />
            </label>
        </div>
        <div class="card-foot py-3 px-4 mt-3 flex">
            <button class="ms-auto btn btn-primary">Sign up</button>
        </div>
    </form>
    <div class="pt-12 text-center text-gray-400">
        Already got an account?
        <a href="/log-in" class="text-primary link-hover"> Log in here </a>
    </div>
</main>

<style lang="postcss">
    input[type] {
        @apply w-full;
    }
</style>
