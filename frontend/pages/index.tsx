import { instantMeiliSearch } from "@meilisearch/instant-meilisearch";
import { useTheme } from "next-themes";
import Head from "next/head";
import React, { useState } from "react";
import {
  Hits,
  InstantSearch,
  Pagination,
  SearchBox,
  Stats,
} from "react-instantsearch-dom";
import CourseCard from "../components/CourseCard";

export default function Home() {
  const [searchQuery, setSearchQuery] = useState("");
  const { theme, setTheme } = useTheme();

  const Footer = () => (
    <footer className="w-full md:w-auto border-t">
      <div className="flex justify-center items-center border-slate-200 w-full">
        <div className="mt-10 scale-90 lg:scale-100">
          <p className="text-center text-gray-600">
            Made with ♥ by{" "}
            <a
              href="https://github.com/tyeetale"
              className="underline font-medium text-blue-500"
              target="_blank noreferrer noopener"
            >
              tyeetale
            </a>{" "}
            &{" "}
            <a
              href="https://github.com/nh8157"
              className="underline font-medium text-blue-500"
              target="_blank noreferrer noopener"
            >
              sheldon chen
            </a>{" "}
            using
          </p>
          <div className="flex justify-center items-center">
            <a href="https://www.rust-lang.org/">
              <svg
                className="scale-[0.35] -m-12"
                height="144"
                width="144"
                fill="currentColor"
                aria-hidden="true"
              >
                <path d="m71.05 23.68c-26.06 0-47.27 21.22-47.27 47.27s21.22 47.27 47.27 47.27 47.27-21.22 47.27-47.27-21.22-47.27-47.27-47.27zm-.07 4.2a3.1 3.11 0 0 1 3.02 3.11 3.11 3.11 0 0 1 -6.22 0 3.11 3.11 0 0 1 3.2-3.11zm7.12 5.12a38.27 38.27 0 0 1 26.2 18.66l-3.67 8.28c-.63 1.43.02 3.11 1.44 3.75l7.06 3.13a38.27 38.27 0 0 1 .08 6.64h-3.93c-.39 0-.55.26-.55.64v1.8c0 4.24-2.39 5.17-4.49 5.4-2 .23-4.21-.84-4.49-2.06-1.18-6.63-3.14-8.04-6.24-10.49 3.85-2.44 7.85-6.05 7.85-10.87 0-5.21-3.57-8.49-6-10.1-3.42-2.25-7.2-2.7-8.22-2.7h-40.6a38.27 38.27 0 0 1 21.41-12.08l4.79 5.02c1.08 1.13 2.87 1.18 4 .09zm-44.2 23.02a3.11 3.11 0 0 1 3.02 3.11 3.11 3.11 0 0 1 -6.22 0 3.11 3.11 0 0 1 3.2-3.11zm74.15.14a3.11 3.11 0 0 1 3.02 3.11 3.11 3.11 0 0 1 -6.22 0 3.11 3.11 0 0 1 3.2-3.11zm-68.29.5h5.42v24.44h-10.94a38.27 38.27 0 0 1 -1.24-14.61l6.7-2.98c1.43-.64 2.08-2.31 1.44-3.74zm22.62.26h12.91c.67 0 4.71.77 4.71 3.8 0 2.51-3.1 3.41-5.65 3.41h-11.98zm0 17.56h9.89c.9 0 4.83.26 6.08 5.28.39 1.54 1.26 6.56 1.85 8.17.59 1.8 2.98 5.4 5.53 5.4h16.14a38.27 38.27 0 0 1 -3.54 4.1l-6.57-1.41c-1.53-.33-3.04.65-3.37 2.18l-1.56 7.28a38.27 38.27 0 0 1 -31.91-.15l-1.56-7.28c-.33-1.53-1.83-2.51-3.36-2.18l-6.43 1.38a38.27 38.27 0 0 1 -3.32-3.92h31.27c.35 0 .59-.06.59-.39v-11.06c0-.32-.24-.39-.59-.39h-9.15zm-14.43 25.33a3.11 3.11 0 0 1 3.02 3.11 3.11 3.11 0 0 1 -6.22 0 3.11 3.11 0 0 1 3.2-3.11zm46.05.14a3.11 3.11 0 0 1 3.02 3.11 3.11 3.11 0 0 1 -6.22 0 3.11 3.11 0 0 1 3.2-3.11z" />
                <path
                  d="m115.68 70.95a44.63 44.63 0 0 1 -44.63 44.63 44.63 44.63 0 0 1 -44.63-44.63 44.63 44.63 0 0 1 44.63-44.63 44.63 44.63 0 0 1 44.63 44.63zm-.84-4.31 6.96 4.31-6.96 4.31 5.98 5.59-7.66 2.87 4.78 6.65-8.09 1.32 3.4 7.46-8.19-.29 1.88 7.98-7.98-1.88.29 8.19-7.46-3.4-1.32 8.09-6.65-4.78-2.87 7.66-5.59-5.98-4.31 6.96-4.31-6.96-5.59 5.98-2.87-7.66-6.65 4.78-1.32-8.09-7.46 3.4.29-8.19-7.98 1.88 1.88-7.98-8.19.29 3.4-7.46-8.09-1.32 4.78-6.65-7.66-2.87 5.98-5.59-6.96-4.31 6.96-4.31-5.98-5.59 7.66-2.87-4.78-6.65 8.09-1.32-3.4-7.46 8.19.29-1.88-7.98 7.98 1.88-.29-8.19 7.46 3.4 1.32-8.09 6.65 4.78 2.87-7.66 5.59 5.98 4.31-6.96 4.31 6.96 5.59-5.98 2.87 7.66 6.65-4.78 1.32 8.09 7.46-3.4-.29 8.19 7.98-1.88-1.88 7.98 8.19-.29-3.4 7.46 8.09 1.32-4.78 6.65 7.66 2.87z"
                  fillRule="evenodd"
                  stroke="currentColor"
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth="3"
                />
              </svg>
            </a>
            ,
            <a href="https://nextjs.org/">
              <img
                src={
                  "https://velog.velcdn.com/images/tjseocld/post/10238718-d9c5-4fb7-a038-e38d221572ed/nextjs.png"
                }
                alt="Nextjs"
                className="pb-1 pt-0.5 object-scale-down h-10 w-10"
              />
            </a>
            ,
            <a href="https://www.meilisearch.com/">
              <img
                src={
                  "https://avatars.githubusercontent.com/u/43250847?s=200&v=4"
                }
                alt="Meilisearch"
                className="pb-1 pt-1 object-scale-down h-20 w-20"
              />
            </a>
            ,
            <a href="https://vercel.com/">
              <img
                src={
                  "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRnjSqXz_mKhvp-05665z75rscs15in6GXTGuC9GlBnp5-AoUFLnw9Or6MfcZmewqW331w&usqp=CAU"
                }
                alt="Vercel"
                className="pb-1 pt-1 object-scale-down h-14 w-14"
              />
            </a>
          </div>
        </div>
      </div>
    </footer>
  );

  const searchClient = instantMeiliSearch(
    "https://ms-53f05ab45a7a-999.nyc.meilisearch.io",
    process.env.NEXT_PUBLIC_MEILI_KEY
  );

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>): void => {
    setSearchQuery(e.target.value);
  };

  return (
    <>
      <Head>
        <title>NYUClassesFromXYZ</title>
        <meta
          name="description"
          content="an NYU search that won't take 2 hours"
        />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <InstantSearch indexName="course" searchClient={searchClient}>
        {searchQuery == "" ? (
          <>
            <main className="min-h-screen flex flex-col justify-center items-center pt-5">
              <div className="md:w-10/12 lg:w-6/12">
                <h1 className="text-center font-extrabold text-transparent xl:text-5xl lg:text-4xl md:text-3xl text-2xl -my-4 bg-clip-text bg-gradient-to-r from-purple-600 to-pink-600">
                  nyuclassesfrom.xyz
                </h1>
                <div className="scale-90 lg:scale-100">
                  <div className="flex flex-nowrap items-center my-5 md:my-8 lg:my-12 elative">
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      fill="none"
                      viewBox="0 0 24 24"
                      strokeWidth="1.5"
                      stroke="currentColor"
                      className="w-6 h-6 absolute z-10 ml-4"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z"
                      />
                    </svg>

                    <input
                      onChange={handleChange}
                      className="searchbar w-full bg-inherit pl-12 hover:bg-gray-100 dark:hover:bg-darkPurple border-2 border-gray-200 py-4 px-8 rounded-full"
                      placeholder={searchQuery === "" ? "Search…" : searchQuery}
                    />
                  </div>
                  <p className="text-center text-gray-500">
                    try words, phrases, titles, subjects, schools, course
                    numbers, instructor names, grading, components, and more.
                    You can also look for exact phrases and prefix matches.
                  </p>
                </div>

                <div className="flex flex-nowrap justify-center mt-4 space-x-4">
                  {/* <a
                    href="https://github.com/tyeetale/nyuclassesfrom.xyz"
                    className="text-slate-400 hover:text-slate-500 dark:hover:text-slate-300"
                  >
                    <span className="sr-only">nyuclassesfromxyz on GitHub</span>
                    <svg
                      viewBox="0 0 16 16"
                      className="w-10 h-10"
                      fill="currentColor"
                      aria-hidden="true"
                    >
                      <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z" />
                    </svg>
                  </a> */}
                  <button
                    onClick={() =>
                      setTheme(theme === "dark" ? "light" : "dark")
                    }
                    className="text-slate-400 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-700 focus:outline-none ring-2 ring-slate-200 dark:ring-slate-700 rounded-lg text-sm p-2.5"
                  >
                    {theme === "dark" ? (
                      <svg
                        className="w-5 h-5"
                        fill="currentColor"
                        viewBox="0 0 20 20"
                        xmlns="http://www.w3.org/2000/svg"
                      >
                        <path
                          d="M10 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4 8a4 4 0 11-8 0 4 4 0 018 0zm-.464 4.95l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 0 00-1.414 1.414zm2.12-10.607a1 1 0 010 1.414l-.706.707a1 1 0 11-1.414-1.414l.707-.707a1 1 0 011.414 0zM17 11a1 1 0 100-2h-1a1 1 0 100 2h1zm-7 4a1 1 0 011 1v1a1 1 0 11-2 0v-1a1 1 0 011-1zM5.05 6.464A1 1 0 106.465 5.05l-.708-.707a1 1 0 00-1.414 1.414l.707.707zm1.414 8.486l-.707.707a1 1 0 01-1.414-1.414l.707-.707a1 1 0 011.414 1.414zM4 11a1 1 0 100-2H3a1 1 0 000 2h1z"
                          fillRule="evenodd"
                          clipRule="evenodd"
                        ></path>
                      </svg>
                    ) : (
                      <svg
                        className="w-5 h-5"
                        fill="currentColor"
                        viewBox="0 0 20 20"
                        xmlns="http://www.w3.org/2000/svg"
                      >
                        <path d="M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z"></path>
                      </svg>
                    )}
                  </button>
                </div>
              </div>
            </main>

            <Footer />
          </>
        ) : (
          <>
            <div className="mx-6 lg:w-full">
              <header className="md:sticky top-0  bg-white dark:bg-darkPurple mb-6">
                <div className="container mx-auto py-4 space-y-4 ">
                  <div className="flex justify-between items-center">
                    <h1 className=" font-extrabold text-transparent text-2xl lg:text-3xl bg-clip-text bg-gradient-to-r from-purple-600 to-pink-600">
                      nyuclassesfrom.xyz
                    </h1>

                    <div className="flex space-x-4">
                      {/* <a
                      href="https://github.com/tyeetale/nyuclassesfrom.xyz"
                      className="text-slate-400 hover:text-slate-500 dark:hover:text-slate-300"
                    >
                      <span className="sr-only">
                        nyuclassesfromxyz on GitHub
                      </span>
                      <svg
                        viewBox="0 0 16 16"
                        className="w-10 h-10"
                        fill="currentColor"
                        aria-hidden="true"
                      >
                        <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z" />
                      </svg>
                    </a> */}
                      <button
                        onClick={() =>
                          setTheme(theme === "dark" ? "light" : "dark")
                        }
                        className="text-slate-400 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-700 focus:outline-none ring-2 ring-slate-200 dark:ring-slate-700 rounded-lg text-sm w-10 h-10 px-2.5"
                      >
                        {theme === "dark" ? (
                          <svg
                            className="w-5 h-5"
                            fill="currentColor"
                            viewBox="0 0 20 20"
                            xmlns="http://www.w3.org/2000/svg"
                          >
                            <path
                              d="M10 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4 8a4 4 0 11-8 0 4 4 0 018 0zm-.464 4.95l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 0 00-1.414 1.414zm2.12-10.607a1 1 0 010 1.414l-.706.707a1 1 0 11-1.414-1.414l.707-.707a1 1 0 011.414 0zM17 11a1 1 0 100-2h-1a1 1 0 100 2h1zm-7 4a1 1 0 011 1v1a1 1 0 11-2 0v-1a1 1 0 011-1zM5.05 6.464A1 1 0 106.465 5.05l-.708-.707a1 1 0 00-1.414 1.414l.707.707zm1.414 8.486l-.707.707a1 1 0 01-1.414-1.414l.707-.707a1 1 0 011.414 1.414zM4 11a1 1 0 100-2H3a1 1 0 000 2h1z"
                              fillRule="evenodd"
                              clipRule="evenodd"
                            ></path>
                          </svg>
                        ) : (
                          <svg
                            className="w-5 h-5"
                            fill="currentColor"
                            viewBox="0 0 20 20"
                            xmlns="http://www.w3.org/2000/svg"
                          >
                            <path d="M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z"></path>
                          </svg>
                        )}
                      </button>
                    </div>
                  </div>
                  <div className="text-md space-y-2">
                    <p>
                      Try words, phrases, titles, subjects, schools, course
                      numbers, instructor names, grading, components, and more.
                      You can also look for exact phrases and prefix matches.
                    </p>
                    <p>
                      Filter by adding more verbose words like "Fall 2022" or
                      "Lecture".
                    </p>
                  </div>

                  <SearchBox autoFocus defaultRefinement={searchQuery} />
                  <Stats />
                </div>
              </header>
              <main className="container mx-auto mb-10 space-y-5">
                <Hits hitComponent={CourseCard as any} />
                <div className="flex justify-center">
                  <Pagination showLast={true} />
                </div>
              </main>
            </div>
            <Footer />
          </>
        )}
      </InstantSearch>
    </>
  );
}
