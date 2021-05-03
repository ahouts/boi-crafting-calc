import * as React from 'react'
import { render } from 'react-dom'
import { Provider } from 'react-redux'
import { App } from './components/app'
import store from './store'

render(
  <React.StrictMode>
    <Provider store={store}>
      <App />
    </Provider>
  </React.StrictMode>,
  document.getElementById('root'),
)
