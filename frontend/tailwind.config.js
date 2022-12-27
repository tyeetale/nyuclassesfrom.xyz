module.exports = {
  darkMode: "class",
  purge: {
    enabled: true,
    content: [
      "./pages/**/*.{js,ts,jsx,tsx}",
      "./components/**/*.{js,ts,jsx,tsx}",
    ],
    options: {
      safeList: ["dark"],
    },
  },
  theme: {
    typography: (theme) => ({}),
    extend: {
      colors: {
        darkPurple: "#070412",
      },
      typography: (theme) => ({
        dark: {
          css: {
            color: "#F5F5F5",
          },
        },
      }),
    },
  },
  variants: {
    typography: ["dark"],
  },
  plugins: [require("@tailwindcss/typography")],
};
