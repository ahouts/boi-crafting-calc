import * as React from 'react'
import { ItemId } from '../../../pkg'
import { import_all } from '../../image_utils'
import { useDispatch, useSelector } from 'react-redux'
import { get_recipes, select_crafter } from '../../slices/crafter_slice'

const images = import_all(require.context('./img', false, /\.(png|jpe?g|svg)$/))

export type ItemArgs = {
  item_id: ItemId,
}

export function Item({ item_id }: ItemArgs): React.ReactElement {
  const selected_item_id = useSelector(select_crafter).recipes?.item_id
  const dispatch = useDispatch()

  const bg_color = selected_item_id === item_id ? 'bg-blue-900' : 'hover:bg-gray-800'

  return (
    <img
      alt={`item ${item_id}`}
      src={images[`collectibles_${item_id.toString().padStart(3, '0')}.png`]}
      className={`flex-none object-contain w-16 h-16 ${bg_color} rounded-xl`}
      onClick={() => dispatch(get_recipes(item_id))}
    />
  )
}
