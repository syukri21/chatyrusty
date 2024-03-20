/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: [
    // include all rust, html and style files in the src directory
    "./src/**/*.{rs,html,css}",
    // include all html files in the output (dist) directory
    "./dist/**/*.html",
  ],
  theme: {
    extend: {

    },
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/typography'),

  ],
}

