import * as React from 'react'
import { Pickup as TPickup } from '../../../pkg'
import { import_all } from '../../image_utils'
import { string_or_empty } from '../../util/defaults'

const images = import_all(require.context('./img', false, /\.(png|jpe?g|svg)$/))

export type PickupImageParams = {
  pickup: TPickup,
  classes?: string,
}

export function PickupImage({ pickup, classes }: PickupImageParams): React.ReactElement {
  return (
    <img className={`object-contain pr-3 ${string_or_empty(classes)}`} src={images[`${pickup}.png`]}
         alt={pickup as unknown as string} />
  )
}