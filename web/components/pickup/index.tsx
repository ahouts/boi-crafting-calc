import * as React from 'react'
import { Pickup as TPickup } from '../../../pkg'
import { useDispatch, useSelector } from 'react-redux'
import { add_pickup, remove_pickup, select_crafter } from '../../reducers/crafter_slice'
import { FaMinus, FaPlus } from 'react-icons/fa'
import { import_all } from '../../image_utils'

const images = import_all(require.context('./img', false, /\.(png|jpe?g|svg)$/))

export type PickupArgs = {
  pickup: TPickup,
}

export function Pickup({ pickup }: PickupArgs): React.ReactElement {
  const state = useSelector(select_crafter)
  const dispatch = useDispatch()

  return (
    <div className={'p-4 my-1 w-54 max-w-sm mx-auto bg-white rounded-xl flex shadow-md space-x-2'}>
      <img className={'object-contain pr-3'} src={images[`${pickup}.png`]} alt={pickup as unknown as string} />
      <button className={'p-3 rounded-xl shadow-md space-x-1 flex-shrink'} onClick={() => {
        dispatch(remove_pickup(pickup))
      }}><FaMinus />
      </button>
      <p className={'p-2 text-xl text-gray-500 flex-shrink'}>{state.pickups[pickup] ?? 0}</p>
      <button className={'p-3 rounded-xl shadow-md space-x-1 flex-shrink'} onClick={() => {
        dispatch(add_pickup(pickup))
      }}><FaPlus />
      </button>
    </div>
  )
}
