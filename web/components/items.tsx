import * as React from 'react'
import { useSelector } from 'react-redux'
import { Item } from './item'
import { select_crafter } from '../reducers/crafter_slice'

export function Items(): React.ReactElement {
  const state = useSelector(select_crafter)

  return (
    <div>
      {state.items.map(item => (
        <Item key={item} item_id={item} />
      ))}
    </div>
  )
}
