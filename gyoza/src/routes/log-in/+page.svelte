<script lang="ts">
    import {logInMutation} from "$lib/backend";
    import {useQueryClient} from "@tanstack/svelte-query";

    const logIn = logInMutation(useQueryClient());

    let username_or_email = "";
    let password = "";

    function submit(e: SubmitEvent) {
        e.preventDefault();
        e.stopPropagation();
        $logIn.mutate({username_or_email, password});
    }
</script>

<h1>Log In</h1>

<form on:submit={submit}>
    <label>
        Username or Email
        <input
            name="username_or_email"
            type="text"
            bind:value={username_or_email}
        />
    </label>
    <label>
        Password
        <input name="password" type="password" bind:value={password} />
    </label>
    <button>Log in</button>
</form>
