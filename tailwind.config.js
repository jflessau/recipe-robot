/** @type {import('tailwindcss').Config} */

module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    colors: {
      bg: "#fcecbc",
      fg: "#fafbff",
      contrast: "#04152b",
      attention: "#e90157",
      info: "#0e62bb",
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
        s: "0.9rem",
        m: "1rem",
        l: "1.1rem",
        xl: "1.6rem",
      },
    },
  },
  plugins: [],
};
