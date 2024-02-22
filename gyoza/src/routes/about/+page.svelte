<svelte:head>
    <title>About</title>
    <meta name="description" content="What in the world is izakaya.dev?" />
</svelte:head>

<main class="max-w-7xl mx-auto flex flex-col p-4">
    <section>
        <div>
            What is this website?
            <a href="https://github.com/ruksi/izakaya">
                TL;DR: a playground project
            </a>
        </div>
        <div>
            Made with love by <a href="https://ruk.si/">Ruksi</a>
        </div>
        <div>
            License:
            <a href="https://github.com/ruksi/izakaya/blob/main/LICENSE">
                MIT
            </a>
        </div>
    </section>

    <section>
        <div class="text-zinc-50">
            Multicolor emojis by
            <a href="https://openmoji.org/">OpenMoji</a>
        </div>
        <div>
            License:
            <a href="https://creativecommons.org/licenses/by-sa/4.0/#">
                CC BY-SA 4.0
            </a>
        </div>
    </section>

    <section>
        <div class="text-zinc-50">
            Monochrome interface icons by
            <a href="https://fontawesome.com/license/free">
                Font Awesome
            </a>
        </div>
        <div>
            License:
            <a href="https://creativecommons.org/licenses/by-sa/4.0/#">
                CC BY-SA 4.0
            </a>
        </div>
    </section>
</main>

<style lang="postcss">
    section {
        @apply mb-4;
    }
    a {
        @apply text-purple-300;
    }
</style>
