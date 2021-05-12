import * as React from 'react'
import { ItemId } from '../../../pkg'
import { import_all } from '../../image_utils'
import { useDispatch } from 'react-redux'
import { get_recipes } from '../../slices/crafter_slice'

const images = import_all(require.context('./img', false, /\.(png|jpe?g|svg)$/))

export type ItemArgs = {
  item_id: ItemId,
}

export function Item({ item_id }: ItemArgs): React.ReactElement {
  const dispatch = useDispatch()

  return (
    <img
      alt={`item ${item_id}`}
      src={images[`collectibles_${item_id.toString().padStart(3, '0')}.png`]}
      className={'flex-none object-contain w-16 h-16'}
      onClick={() => dispatch(get_recipes(item_id))}
    />
  )
}
