import * as React from 'react'
import { Pickup as TPickup } from '../../../pkg'
import { useDispatch, useSelector } from 'react-redux'
import { add_pickup, remove_pickup, select_crafter } from '../../slices/crafter_slice'
import { FaMinus, FaPlus } from 'react-icons/fa'
import { PickupImage } from './pickup_image'

export type PickupArgs = {
  pickup: TPickup,
}

export function Pickup({ pickup }: PickupArgs): React.ReactElement {
  const state = useSelector(select_crafter)
  const dispatch = useDispatch()

  return (
    <div className={'p-4 my-1 w-54 max-w-sm mx-auto bg-gray-700 rounded-xl flex shadow-md space-x-2'}>
      <PickupImage pickup={pickup} />
      <button className={
        'p-3 rounded-xl shadow-md space-x-1 flex-shrink bg-gray-800 hover:bg-gray-900 focus:outline-none'
      }
              onClick={() => {
                dispatch(remove_pickup(pickup))
              }}>
        <FaMinus />
      </button>
      <p className={'p-2 text-xl flex-shrink'}>{state.pickups[pickup] ?? 0}</p>
      <button className={
        'p-3 rounded-xl shadow-md space-x-1 flex-shrink bg-gray-800 hover:bg-gray-900 focus:outline-none'
      }
              onClick={() => {
                dispatch(add_pickup(pickup))
              }}>
        <FaPlus />
      </button>
    </div>
  )
}
