<script>
    import Profile from "$lib/user/Profile.svelte";
    import Emails from "$lib/user/Emails.svelte";
    import Access from "$lib/user/Access.svelte";
</script>

<svelte:head>
    <title>Settings</title>
    <meta name="description" content="User settings page">
</svelte:head>

<main class="max-w-7xl mx-auto p-4 mb-16">
    <h1 class="text-5xl">Settings</h1>
    <div class="flex flex-col gap-4 mt-4">
        <section>
            <h2>Profile</h2>
            <Profile />
        </section>
        <section>
            <h2>Emails</h2>
            <Emails />
        </section>
        <section>
            <h2>Access</h2>
            <Access />
        </section>
    </div>
</main>

<style lang="postcss">
    section {
        @apply mt-2;
    }
    section h2 {
        @apply text-2xl mb-1;
    }
</style>
