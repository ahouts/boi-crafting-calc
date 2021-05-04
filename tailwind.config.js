module.exports = {
  purge: {
    mode: 'all',
    preserveHtmlElements: false,
    content: [
      './web/**/*.tsx',
      './web/**/*.html',
    ],
    options: {
      keyframes: true,
    },
  },
  darkMode: false,
  theme: {
    extend: {},
  },
  variants: {
    extend: {},
  },
  plugins: [],
}
