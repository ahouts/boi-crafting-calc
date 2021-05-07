import * as React from 'react'
import { render } from 'react-dom'
import { Provider } from 'react-redux'
import { App } from './components/app'
import store from './store'
import './styles.css'

render(
  <React.StrictMode>
    <Provider store={store}>
      <div className={'min-w-screen min-h-screen bg-gray-900 text-white'}>
        <App />
      </div>
    </Provider>
  </React.StrictMode>,
  document.getElementById('root'),
)
