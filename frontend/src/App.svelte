<script>
  let searchValue = "";
</script>

<main>
  <div class="flex h-screen">
    <div class="m-auto">
      <h1
        class="p-1 font-extrabold text-transparent text-6xl bg-clip-text bg-gradient-to-r from-purple-600 to-pink-600"
      >
        nyuclassesfrom.xyz
      </h1>
      <input class="mt-3" bind:value={searchValue} />
      <p>Input: {searchValue}</p>
      <p
        class="p-1 text-transparent text-md bg-clip-text bg-gradient-to-r from-purple-600 to-pink-600"
      >
        try words, phrases, titles, subjects, schools, course numbers,
        instructor names, grading, components, and more. You can also look for
        exact phrases and prefix matches
      </p>
      <div class="flex items-center border-slate-200  dark:border-slate-800">
        <!-- ADD: <ThemeToggle /> -->
        <a
          href="https://github.com/tyeetale/nyuclassesfrom.xyz"
          class="text-slate-400 hover:text-slate-500 dark:hover:text-slate-300"
        >
          <span class="sr-only">nyuclassesfromxyz on GitHub</span>
          <svg
            viewBox="0 0 16 16"
            class="w-5 h-5"
            fill="currentColor"
            aria-hidden="true"
          >
            <path
              d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"
            />
          </svg>
        </a>
      </div>
    </div>
  </div>
</main>

<footer>
  <p class="md:text-center md:mx-auto max-w-screen-sm" />
  Made with ♥ by {" "}<a
    href="https://github.com/tyeetale"
    class="underline font-medium text-blue-500"
    target="_blank noreferrer noopener">tyeetale</a
  >
  &
  <a
    href="https://github.com/nh8157"
    class="underline font-medium text-blue-500"
    target="_blank noreferrer noopener">sheldon chen</a
  >
  <div class="flex items-center">
    using
    <a href="https://www.rust-lang.org/"
      ><img
        src={"https://www.rust-lang.org/static/images/rust-logo-blk.svg"}
        alt="Rust"
        class="pb-1 pt-0.5 object-scale-down h-12 w-12"
      /></a
    >,<a href="https://svelte.dev"
      ><img
        src={"https://upload.wikimedia.org/wikipedia/commons/thumb/1/1b/Svelte_Logo.svg/1702px-Svelte_Logo.svg.png"}
        alt="Svelte"
        class="pb-1 pt-0.5 object-scale-down h-10 w-10"
      /></a
    >
    >, indexed with
    <a href="https://redis.io"
      ><img
        src={"https://download.logo.wine/logo/Redis/Redis-Logo.wine.png"}
        alt="Redis"
        class="pb-1 pt-1 object-scale-down h-20 w-20"
      /></a
    >, deployed to
    <a href="https://vercel.com/"
      ><img
        src={"https://mms.businesswire.com/media/20211123005573/en/929867/23/vercel-logo-freelogovectors.net.jpg"}
        alt="Vercel"
        class="ml-2 pb-1 pt-0.5 object-scale-down h-20 w-20"
      /></a
    >
  </div>
</footer>

<style>
  :global(body) {
    background-color: rgb(248 250 252);
  }
</style>
