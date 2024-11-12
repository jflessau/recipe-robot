/** @type {import('tailwindcss').Config} */

module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    colors: {
      bg: "#fcecbc",
      fg: "#fafbff",
      mid: "#f3d584",
      contrast: "#04152b",
      attention: "#a519ff",
      info: "#09488a",

      color: "#04152b",
      "color-inverted": "#fafbff",
    },
    extend: {
      fontFamily: {
        noto: ["Noto Serif", "sans-serif"],
      },
      fontWeight: {
        thin: "300",
        regular: "400",
        bold: "600",
        black: "800",
      },
      fontSize: {
        xs: "0.6rem",
        s: "0.8rem",
        m: "1rem",
        l: "1.1rem",
        xl: "1.6rem",
      },
    },
  },
  plugins: [],
};
