<script lang="ts">
    import {logInMutation} from "$lib/backend";
    import {useQueryClient} from "@tanstack/svelte-query";

    const logIn = logInMutation(useQueryClient());

    let username_or_email = "";
    let password = "";

    function handleSubmit(e: SubmitEvent) {
        e.preventDefault();
        e.stopPropagation();
        $logIn.mutate({username_or_email, password});
    }
</script>

<h1 class="hidden">Log In</h1>

<form on:submit={handleSubmit}>
    <label>
        Username or Email
        <input name="username_or_email" type="text" bind:value={username_or_email} />
    </label>
    <label>
        Password
        <input name="password" type="password" bind:value={password} />
    </label>
    <button>Log in</button>
</form>

<style lang="postcss">
    form {
        @apply flex flex-col gap-3 max-w-xl mx-auto pt-10;
    }
    input[type="text"],
    input[type="password"] {
        @apply form-input p-1 bg-zinc-800 rounded;
        @apply focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-zinc-300;
    }
</style>
