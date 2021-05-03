import * as React from 'react'
import { Pickup as TPickup } from '../../pkg'
import { Pickup } from './pickup'

export function Pickups(): React.ReactElement {
  return (
    <div>
      {Object.values(TPickup)
        .filter(key => isNaN(Number(key)))
        .map(k => k as TPickup)
        .map(pickup => (
          <Pickup key={pickup} pickup={pickup} />
        ))}
    </div>
  )
}
