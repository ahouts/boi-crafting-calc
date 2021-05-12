import * as React from 'react'
import { useSelector } from 'react-redux'
import { Item } from './item'
import { select_crafter } from '../slices/crafter_slice'
import { string_or_empty } from '../util/defaults'

export interface ItemsParams {
  class_names?: string
}

export function Items({ class_names }: ItemsParams = {}): React.ReactElement {
  const state = useSelector(select_crafter)

  return (
    <div className={`w-auto bg-gray-700 ${string_or_empty(class_names)}`}>
      <div className={'flex flex-wrap w-auto'}>
        {state.items.map(item => (
          <Item key={item} item_id={item} />
        ))}
      </div>
    </div>
  )
}
