<script>
    import svgRedPaperLantern from '$lib/site/red-paper-lantern.svg';
    import svgPurpleHeart from '$lib/site/purple-heart.svg';
</script>

<svelte:head>
    <title>Izakaya</title>
    <meta name="description" content="Explore Izakaya - a state-of-the-art web application!" />
</svelte:head>

<main class="flex flex-col max-w-7xl mx-auto p-4">
    <div class="
        mt-20 sm:mt-32 xl:mt-52
        mb-8  sm:mb-16 xl:mb-24
    ">
        <div class="flex">
            <div class="
                flex-shrink-0
                basis-12 sm:basis-16 md:basis-20
                flex flex-col justify-center
                mr-4 xl:mr-6
            ">
                <picture>
                    <img
                        src={svgRedPaperLantern}
                        alt="A red paper lantern"
                        height="80"
                        width="80"
                        class="hero-lantern"
                    />
                </picture>
            </div>
            <div class="flex flex-col justify-center">
                <h1 class="
                    text-xl sm:text-3xl md:text-4xl lg:text-5xl xl:text-6xl
                    leading-normal font-bold
                ">
                    This is Izakaya landing&nbsp;page
                </h1>
            </div>
        </div>
        <div class="
            mt-2 md:mt-3 xl:mt-5
            text-zinc-400 leading-tight font-normal
            text-sm sm:text-base md:text-lg lg:text-xl
        ">
            with Vite + SvelteKit + TanStack&nbsp;Query
        </div>
    </div>

    <section class="flex justify-center">
        <div class="flex flex-col text-center">
            <div class="
                text-base xl:text-xl
                font-normal
            ">Social proof is excellent for assurance.
            </div>
            <div class="
                text-xs xl:text-sm
                text-zinc-400 font-light
            ">
                — Bob Bobbins in <cite>the Big Company</cite>
            </div>
        </div>
    </section>

    <section>
        <header class="text-orange-300/95">
            Core Message
        </header>
        <p>
            This is why you should use this service or product, and how it will
            change your life forever.
        </p>
        <div class="
            flex flex-col
            gap-2.5 lg:gap-5
            text-lg md:text-2xl
            leading-snug text-neutral-500
            mt-5
        ">
            <div>
                <div class="card p-4">
                    <span class="text-neutral-100">Major Feature</span>
                    helps you do thing we were just talking about above.
                </div>
            </div>
            <div class="
                flex
                gap-2.5 lg:gap-5
            ">
                <div class="card p-4">
                    <span class="text-neutral-100">Minor Feature</span>
                    helps you do thing we were just talking about above.

                </div>
                <div class="card p-4">
                    <span class="text-neutral-100">Minor Feature</span>
                    helps you do thing we were just talking about above.
                </div>
            </div>
        </div>
    </section>

    <section>
        <header class="text-amber-300/95">
            Additional Benefits
        </header>
        <p>
            You would probably have some inspirational images or photographs all around.
        </p>
        <div class="
            flex flex-col
            gap-2.5 lg:gap-5
            text-lg md:text-2xl
            leading-snug text-neutral-500
            mt-5
        ">
            <div class="
                flex
                gap-2.5 lg:gap-5
            ">
                <div class="card p-4">
                    <span class="text-neutral-100">Left Benefit</span>
                    helps you do thing we were just talking about above.
                </div>
                <div class="card p-4">
                    <span class="text-neutral-100">Right Benefit</span>
                    helps you do thing we were just talking about above.
                </div>
            </div>
            <div>
                <div class="card p-4">
                    <span class="text-neutral-100">Big Benefit</span>
                    helps you do thing we were just talking about above.
                </div>
            </div>
        </div>
    </section>

    <section class="flex flex-col gap-4 max-w-72">
        <div class="mb-2">
            <div class="text-2xl italic">Izakaya</div>
        </div>
        <div class="text-neutral-400">
            Register for <span class="text-neutral-200">Izakaya</span> to get a closer look at all the awesome features.
        </div>
        <div class="text-neutral-400 italic">
            <div>You don't have to use your actual&nbsp;email.</div>
            <div>They aren't verified.</div>
        </div>
        <div class="mt-3">
            <a href="/sign-up" class="ms-auto btn btn-purple">Sign Up</a>
        </div>
    </section>

    <div class="flex flex-col gap-4 text-center mt-4 mb-4">
        <div class="flex justify-center">
            <picture class="inline-block">
                <img
                    src={svgRedPaperLantern}
                    alt="A red paper lantern"
                    height="80"
                    width="80"
                    class="footer-lantern"
                />
            </picture>
        </div>
        <div>
            Made with
            <div class="inline-flex align-middle">
                <picture>
                    <img
                        src={svgPurpleHeart}
                        alt="love"
                        title="love"
                        height="24"
                        width="24"
                    />
                </picture>
            </div>
            by <a href="https://ruk.si/" class="text-purple-400">Ruksi</a>
        </div>
    </div>
</main>

<style lang="postcss">
    .hero-lantern {
        background-color: rgba(255, 183, 120, 0.7);
        border-radius: 50%;
        box-shadow: 0 0 0.5rem 0.25rem rgba(255, 183, 120, 0.4);
    }

    section {
        @apply my-8 sm:my-16 xl:my-24;
    }

    section header {
        @apply mb-1 md:mb-4;
    }

    section header,
    section p {
        @apply text-xl sm:text-2xl md:text-3xl lg:text-4xl xl:text-5xl;
        @apply font-light leading-tight;
    }

    .footer-lantern {
        background-color: rgba(255, 183, 120, 0.7);
        border-radius: 50%;
        box-shadow: 0 0 0.5rem 0.25rem rgba(255, 183, 120, 0.4);
        will-change: transform, transform-origin;
        animation: swing 4s ease-in-out infinite;
    }

    @keyframes swing {
        0% {
            transform: rotate(8deg);
            transform-origin: center top;
        }
        50% {
            transform: rotate(-8deg);
            transform-origin: center top;
        }
        100% {
            transform: rotate(8deg);
            transform-origin: center top;
        }
    }
</style>
