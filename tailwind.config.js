/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      colors: {
        whitesmoke: "#f5f5f5",
        "main-color": "#e7d1ba",
      },
    },
  },
  plugins: [],
};
