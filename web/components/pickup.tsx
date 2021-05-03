import * as React from 'react'
import { Pickup as TPickup } from '../../pkg'
import { useDispatch, useSelector } from 'react-redux'
import { add_pickup, remove_pickup, select_crafter } from '../reducers/crafter_slice'

export type PickupArgs = {
  pickup: TPickup,
}

export function Pickup({ pickup }: PickupArgs): React.ReactElement {
  const state = useSelector(select_crafter)
  const dispatch = useDispatch()

  return (
    <div>
      <p>{pickup}</p>
      <button onClick={() => {
        dispatch(remove_pickup(pickup))
      }}>-
      </button>
      <p>{state.pickups[pickup] ?? 0}</p>
      <button onClick={() => {
        dispatch(add_pickup(pickup))
      }}>+
      </button>
    </div>
  )
}
