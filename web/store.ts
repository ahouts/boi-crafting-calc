import { configureStore } from '@reduxjs/toolkit'
import { crafter_slice } from './reducers/crafter_slice'
import thunk from 'redux-thunk'

const store = configureStore({
  reducer: {
    crafter: crafter_slice.reducer,
  },
  middleware: [
    thunk,
  ],
})

export default store
export type RootState = ReturnType<typeof store.getState>
