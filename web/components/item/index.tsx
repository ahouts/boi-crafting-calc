import * as React from 'react'
import { ItemId } from '../../../pkg'
import RequireContext = __WebpackModuleApi.RequireContext

function import_all(r: RequireContext): Record<string, string> {
  const images: Record<string, string> = {}
  r.keys().map((item) => {
    images[item.replace('./', '')] = r(item)
  })
  return images
}

const images = import_all(require.context('./img', false, /\.(png|jpe?g|svg)$/))

export type ItemArgs = {
  item_id: ItemId,
}

export function Item({ item_id }: ItemArgs): React.ReactElement {
  return (
    <img alt={`item ${item_id}`} src={images[`collectibles_${item_id.toString().padStart(3, '0')}.png`]} />
  )
}
