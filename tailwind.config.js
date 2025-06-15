/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
    transform: {
      rs: (content) => content.replace(/(?:^|\s)class:/g, ' '),
    },
  },
  darkMode: 'media', // or 'class' if you prefer manual control
  theme: {
    extend: {
      fontFamily: {
        Geist: ['Geist', 'sans-serif', 'mono'],
      },
    },
  },
  plugins: [],
}
