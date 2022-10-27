<script context="module">
  export function encodeQueryHash(query) {
    return "#" + encodeURIComponent(query).replaceAll("%20", "+");
  }

  export function decodeQueryHash(hash) {
    return decodeURIComponent(hash.slice(1).replaceAll("+", "%20"));
  }
</script>

<script>
  import { onMount } from "svelte";
  import { loop_guard } from "svelte/internal";
  import ThemeSwitch from "./lib/ThemeSwitch/ThemeSwitch.svelte";

  let fetchData = fetch("../../fake-data.json").then((res) => res.json());
  let query = location.hash ? decodeQueryHash(location.hash) : "";
  $: {
    const newUrl = query
      ? encodeQueryHash(query)
      : location.pathname + location.search;
    history.replaceState(null, "", newUrl);
  }

  let landing = query === "";
  $: if (query) landing = false;

  let ay2023 = false;

  // Render courses incrementally in batches of 20 at a time, to avoid slowing
  // down the browser with too many elements at once.
  let showing = 0;
  let showingTimeout = 0;
  // function showMore() {
  //   const len = $data?.courses?.length ?? 0;
  //   if (showing < len) {
  //     showing += Math.min(20, len - showing);
  //     showingTimeout = window.setTimeout(showMore, 100);
  //   }
  // }
  // onMount(() =>
  //   data.subscribe(() => {
  //     window.clearTimeout(showingTimeout);
  //     showing = 0;
  //     showMore();
  //   })
  // );
</script>

<!-- 
{#await fetchData}
  <p>Loading JSON</p>
{:then result}
  <p>{JSON.stringify(result)}</p>
{/await}
 -->

<main class:landing>
  {#if landing}<div class="flex h-screen">
      <div class="m-auto">
        <h1
          class="text-center p-1 font-extrabold text-transparent text-6xl bg-clip-text bg-gradient-to-r from-purple-600 to-pink-600"
        >
          nyuclassesfrom.xyz
        </h1>
        <div class="flex flex-nowrap items-center my-4 relative">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            class="w-6 h-6 absolute z-10 ml-4"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z"
            />
          </svg>

          <input
            class="searchbar w-full bg-inherit pl-12 hover:bg-gray-100 border-2 border-gray-200 py-4 px-8 rounded-full"
            placeholder={landing ? "" : "Search…"}
            bind:value={query}
          />
        </div>

        <p
          class="p-1  text-center text-transparent text-xl bg-clip-text bg-gradient-to-r from-purple-600 to-pink-600"
        >
          try words, phrases, titles, subjects, schools, course numbers,
          <br /> instructor names, grading, components, and more. <br />You can
          also look for exact phrases and prefix matches.
        </p>

        <div class="flex flex-nowrap justify-center mt-4 space-x-4">
          <a
            href="https://github.com/tyeetale/nyuclassesfrom.xyz"
            class="text-slate-400 hover:text-slate-500 dark:hover:text-slate-300"
          >
            <span class="sr-only">nyuclassesfromxyz on GitHub</span>
            <svg
              viewBox="0 0 16 16"
              class="w-10 h-10"
              fill="currentColor"
              aria-hidden="true"
            >
              <path
                d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"
              />
            </svg>
          </a>
          <ThemeSwitch />
        </div>
      </div>
    </div>{/if}

  {#if query !== ""}
    <div>escaping the landing page</div>
    <!-- svelte-ignore a11y-autofocus -->
    <input
      autofocus
      class="searchbar"
      placeholder={landing ? "" : "Search…"}
      bind:value={query}
    />
  {/if}
</main>

{#if landing}<footer class="flex">
    <div class="mt-10 m-auto">
      <p class="text-center">
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
      </p>
      <div class="flex items-center">
        data using
        <a href="https://www.rust-lang.org/">
          <svg
            class="scale-[0.35] -m-12"
            height="144"
            width="144"
            fill="currentColor"
            aria-hidden="true"
            ><path
              d="m71.05 23.68c-26.06 0-47.27 21.22-47.27 47.27s21.22 47.27 47.27 47.27 47.27-21.22 47.27-47.27-21.22-47.27-47.27-47.27zm-.07 4.2a3.1 3.11 0 0 1 3.02 3.11 3.11 3.11 0 0 1 -6.22 0 3.11 3.11 0 0 1 3.2-3.11zm7.12 5.12a38.27 38.27 0 0 1 26.2 18.66l-3.67 8.28c-.63 1.43.02 3.11 1.44 3.75l7.06 3.13a38.27 38.27 0 0 1 .08 6.64h-3.93c-.39 0-.55.26-.55.64v1.8c0 4.24-2.39 5.17-4.49 5.4-2 .23-4.21-.84-4.49-2.06-1.18-6.63-3.14-8.04-6.24-10.49 3.85-2.44 7.85-6.05 7.85-10.87 0-5.21-3.57-8.49-6-10.1-3.42-2.25-7.2-2.7-8.22-2.7h-40.6a38.27 38.27 0 0 1 21.41-12.08l4.79 5.02c1.08 1.13 2.87 1.18 4 .09zm-44.2 23.02a3.11 3.11 0 0 1 3.02 3.11 3.11 3.11 0 0 1 -6.22 0 3.11 3.11 0 0 1 3.2-3.11zm74.15.14a3.11 3.11 0 0 1 3.02 3.11 3.11 3.11 0 0 1 -6.22 0 3.11 3.11 0 0 1 3.2-3.11zm-68.29.5h5.42v24.44h-10.94a38.27 38.27 0 0 1 -1.24-14.61l6.7-2.98c1.43-.64 2.08-2.31 1.44-3.74zm22.62.26h12.91c.67 0 4.71.77 4.71 3.8 0 2.51-3.1 3.41-5.65 3.41h-11.98zm0 17.56h9.89c.9 0 4.83.26 6.08 5.28.39 1.54 1.26 6.56 1.85 8.17.59 1.8 2.98 5.4 5.53 5.4h16.14a38.27 38.27 0 0 1 -3.54 4.1l-6.57-1.41c-1.53-.33-3.04.65-3.37 2.18l-1.56 7.28a38.27 38.27 0 0 1 -31.91-.15l-1.56-7.28c-.33-1.53-1.83-2.51-3.36-2.18l-6.43 1.38a38.27 38.27 0 0 1 -3.32-3.92h31.27c.35 0 .59-.06.59-.39v-11.06c0-.32-.24-.39-.59-.39h-9.15zm-14.43 25.33a3.11 3.11 0 0 1 3.02 3.11 3.11 3.11 0 0 1 -6.22 0 3.11 3.11 0 0 1 3.2-3.11zm46.05.14a3.11 3.11 0 0 1 3.02 3.11 3.11 3.11 0 0 1 -6.22 0 3.11 3.11 0 0 1 3.2-3.11z"
            /><path
              d="m115.68 70.95a44.63 44.63 0 0 1 -44.63 44.63 44.63 44.63 0 0 1 -44.63-44.63 44.63 44.63 0 0 1 44.63-44.63 44.63 44.63 0 0 1 44.63 44.63zm-.84-4.31 6.96 4.31-6.96 4.31 5.98 5.59-7.66 2.87 4.78 6.65-8.09 1.32 3.4 7.46-8.19-.29 1.88 7.98-7.98-1.88.29 8.19-7.46-3.4-1.32 8.09-6.65-4.78-2.87 7.66-5.59-5.98-4.31 6.96-4.31-6.96-5.59 5.98-2.87-7.66-6.65 4.78-1.32-8.09-7.46 3.4.29-8.19-7.98 1.88 1.88-7.98-8.19.29 3.4-7.46-8.09-1.32 4.78-6.65-7.66-2.87 5.98-5.59-6.96-4.31 6.96-4.31-5.98-5.59 7.66-2.87-4.78-6.65 8.09-1.32-3.4-7.46 8.19.29-1.88-7.98 7.98 1.88-.29-8.19 7.46 3.4 1.32-8.09 6.65 4.78 2.87-7.66 5.59 5.98 4.31-6.96 4.31 6.96 5.59-5.98 2.87 7.66 6.65-4.78 1.32 8.09 7.46-3.4-.29 8.19 7.98-1.88-1.88 7.98 8.19-.29-3.4 7.46 8.09 1.32-4.78 6.65 7.66 2.87z"
              fill-rule="evenodd"
              stroke="currentColor"
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="3"
            /></svg
          >
        </a>, frontend applying
        <a href="https://svelte.dev"
          ><img
            src={"https://upload.wikimedia.org/wikipedia/commons/thumb/1/1b/Svelte_Logo.svg/1702px-Svelte_Logo.svg.png"}
            alt="Svelte"
            class="pb-1 pt-0.5 object-scale-down h-10 w-10"
          /></a
        >
        , indexing with
        <a href="https://redis.io"
          ><img
            src={"https://download.logo.wine/logo/Redis/Redis-Logo.wine.png"}
            alt="Redis"
            class="pb-1 pt-1 object-scale-down h-20 w-20"
          /></a
        >, deployed to
        <a href="https://vercel.com/" class="pl-2">
          <svg
            viewBox="0 0 282.72 64"
            class="w-20 h-16"
            fill="currentColor"
            aria-hidden="true"
            ><path
              d="M141.04 16c-11.04 0-19 7.2-19 18s8.96 18 20 18c6.67 0 12.55-2.64 16.19-7.09l-7.65-4.42c-2.02 2.21-5.09 3.5-8.54 3.5-4.79 0-8.86-2.5-10.37-6.5h28.02c.22-1.12.35-2.28.35-3.5 0-10.79-7.96-17.99-19-17.99zm-9.46 14.5c1.25-3.99 4.67-6.5 9.45-6.5 4.79 0 8.21 2.51 9.45 6.5zM248.72 16c-11.04 0-19 7.2-19 18s8.96 18 20 18c6.67 0 12.55-2.64 16.19-7.09l-7.65-4.42c-2.02 2.21-5.09 3.5-8.54 3.5-4.79 0-8.86-2.5-10.37-6.5h28.02c.22-1.12.35-2.28.35-3.5 0-10.79-7.96-17.99-19-17.99zm-9.45 14.5c1.25-3.99 4.67-6.5 9.45-6.5 4.79 0 8.21 2.51 9.45 6.5zM200.24 34c0 6 3.92 10 10 10 4.12 0 7.21-1.87 8.8-4.92l7.68 4.43c-3.18 5.3-9.14 8.49-16.48 8.49-11.05 0-19-7.2-19-18s7.96-18 19-18c7.34 0 13.29 3.19 16.48 8.49l-7.68 4.43c-1.59-3.05-4.68-4.92-8.8-4.92-6.07 0-10 4-10 10zm82.48-29v46h-9V5zM36.95 0L73.9 64H0zm92.38 5l-27.71 48L73.91 5H84.3l17.32 30 17.32-30zm58.91 12v9.69c-1-.29-2.06-.49-3.2-.49-5.81 0-10 4-10 10V51h-9V17h9v9.2c0-5.08 5.91-9.2 13.2-9.2z"
            /></svg
          >
        </a>
      </div>
    </div>
  </footer>{/if}
