import React from 'react'
import ReactDOM from 'react-dom'

import AppRoot from '@/components/approot'

import './index.css'

// :: ---

const __approotElement = document.querySelector('#approot')

ReactDOM.render(
  <React.StrictMode>
    <AppRoot />
  </React.StrictMode>,
  __approotElement
)
