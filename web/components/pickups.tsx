import * as React from 'react'
import { Pickup as TPickup } from '../../pkg'
import { Pickup } from './pickup'
import { string_or_empty } from '../util/defaults'

export interface PickupsParams {
  class_names?: string
}

export function Pickups({ class_names }: PickupsParams = {}): React.ReactElement {
  return (
    <div className={
      `grid grid-cols-4 xl:grid-cols-6 2xl:grid-cols-7 w-auto justify-items-center ${string_or_empty(class_names)}`
    }>
      {Object.values(TPickup)
        .filter(key => isNaN(Number(key)))
        .map(k => k as TPickup)
        .map(pickup => (
          <Pickup key={pickup} pickup={pickup} />
        ))}
    </div>
  )
}
