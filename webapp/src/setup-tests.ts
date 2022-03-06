/* eslint-disable unicorn/prefer-module,@typescript-eslint/no-var-requires */
import '@testing-library/jest-dom'
// import { fetch } from 'whatwg-fetch'
const { fetch } = require('whatwg-fetch')

// :: ---

global.fetch = fetch
