/* eslint-disable unicorn/prefer-module */
module.exports = {
  content: [
    'index.html', // The only flat file entrypoint for the app bundler
    './src/**/*.{ts,tsx,js,jsx}', // Class names may be referenced in JSX
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
