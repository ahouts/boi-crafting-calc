import * as React from 'react'
import { useEffect } from 'react'
import { useDispatch, useSelector } from 'react-redux'
import init, { DeltaCrafter } from '../../pkg/boi_crafting_calc'
import { reset, select_crafter, set } from '../slices/crafter_slice'

const Recipes = React.lazy(() => import('./recipes').then(recipes => ({ default: recipes.Recipes })))
const Pickups = React.lazy(() => import('./pickups').then(pickups => ({ default: pickups.Pickups })))
const Items = React.lazy(() => import('./items').then(items => ({ default: items.Items })))
const itempools_xml_promise = require('raw-loader!../assets/itempools.xml')
const items_metadata_xml_promise = require('raw-loader!../assets/items_metadata.xml')

const ONCE: Array<boolean> = [ false ]

export function App(): React.ReactElement {
  const recipes = useSelector(select_crafter).recipes
  const dispatch = useDispatch()

  useEffect(() => {
    let delta_crafter: DeltaCrafter

    (async () => {
      if (!ONCE[0]) {
        ONCE[0] = true
        await init()
        const { default: itempools_xml } = await itempools_xml_promise
        const { default: items_metadata_xml } = await items_metadata_xml_promise

        delta_crafter = new DeltaCrafter(itempools_xml, items_metadata_xml)
        dispatch(set(delta_crafter))
      }
    })()
  })

  return (
    <React.Suspense fallback={<p>loading...</p>}>
      <div className={'w-screen grid grid-cols-3'}>
        <Recipes classes={recipes === null ? 'hide' : 'col-span-1'} />
        <div className={
          `w-full min-h-screen flex flex-col ${recipes === null ? 'col-span-3' : 'col-span-2'}`
        }>
          <Pickups class_names={'flex-shrink-0'} />
          <button onClick={() => dispatch(reset())} className={
            'p-3 rounded-xl shadow-md space-x-1 flex-shrink capitalize font-bold bg-red-800 hover:bg-red-900 ' +
            'flex-shrink-0 focus:outline-none'
          }>
            reset selection
          </button>
          <Items class_names={'flex-grow'} />
        </div>
      </div>
    </React.Suspense>
  )
}
