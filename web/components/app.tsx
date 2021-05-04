import * as React from 'react'
import { useEffect } from 'react'
import { Pickups } from './pickups'
import { useDispatch } from 'react-redux'
import init, { DeltaCrafter } from '../../pkg/boi_crafting_calc'
import { clear, set } from '../reducers/crafter_slice'
import { Items } from './items'

export function App(): React.ReactElement {
  const dispatch = useDispatch()

  useEffect(() => {
    (async () => {
      await init()

      dispatch(set(new DeltaCrafter()))
    })()

    return () => {
      dispatch(clear())
    }
  })

  return (
    <div className={'w-screen'}>
      <Pickups />
      <Items />
    </div>
  )
}
