import * as React from 'react'
import { useEffect } from 'react'
import { useDispatch } from 'react-redux'
import init, { DeltaCrafter } from '../../pkg/boi_crafting_calc'
import { clear, set } from '../reducers/crafter_slice'

const Pickups = React.lazy(() => import('./pickups').then(pickups => ({ default: pickups.Pickups })))
const Items = React.lazy(() => import('./items').then(items => ({ default: items.Items })))
const itempools_xml_promise = require('raw-loader!../assets/itempools.xml')
const items_metadata_xml_promise = require('raw-loader!../assets/items_metadata.xml')

export function App(): React.ReactElement {
  const dispatch = useDispatch()

  useEffect(() => {
    (async () => {
      await init()
      const { default: itempools_xml } = await itempools_xml_promise
      const { default: items_metadata_xml } = await items_metadata_xml_promise

      dispatch(set(new DeltaCrafter(itempools_xml, items_metadata_xml)))
    })()

    return () => {
      dispatch(clear())
    }
  })

  return (
    <div className={'w-screen'}>
      <React.Suspense fallback={<p>loading...</p>}>
        <Pickups />
        <Items />
      </React.Suspense>
    </div>
  )
}
