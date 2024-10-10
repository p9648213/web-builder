/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: ["class"],
  content: ["./src/**/*.rs", "./assets/**/*.js", "./templates/**/*.stpl"],
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/forms")],
};