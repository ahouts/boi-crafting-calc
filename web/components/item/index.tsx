import * as React from 'react'
import { ItemId } from '../../../pkg'
import { import_all } from '../../image_utils'

const images = import_all(require.context('./img', false, /\.(png|jpe?g|svg)$/))

export type ItemArgs = {
  item_id: ItemId,
}

export function Item({ item_id }: ItemArgs): React.ReactElement {
  return (
    <img
      alt={`item ${item_id}`}
      src={images[`collectibles_${item_id.toString().padStart(3, '0')}.png`]}
    />
  )
}
