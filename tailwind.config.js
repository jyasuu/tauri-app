const resolve = require('path').resolve
// const plugin = require('tailwindcss/plugin')
// const defaultTheme = require('tailwindcss/defaultTheme')

/** @type {import('tailwindcss').Config} */
export default {
  content: [resolve(__dirname, 'index.html'), resolve(__dirname, 'src/**/*.{vue,ts}')],
  theme: {
    extend: {},
  },
  plugins: [],
}

