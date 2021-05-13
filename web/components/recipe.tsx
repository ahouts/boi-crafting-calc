import * as React from 'react'
import { Pickup, Recipe as TRecipe } from '../../pkg'
import { PickupImage } from './pickup/pickup_image'
import { FaHammer } from 'react-icons/fa'
import { useDispatch } from 'react-redux'
import { craft_recipe } from '../slices/crafter_slice'

export type RecipeParams = {
  recipe: TRecipe,
}

export function Recipe({ recipe }: RecipeParams): React.ReactElement {
  const dispatch = useDispatch()
  return (
    <div className={'flex'}>
      <FaHammer className={'w-12 h-12'} onClick={() => dispatch(craft_recipe(recipe))} />
      <div className={'grid grid-cols-4 auto-rows-auto bg-gray-800'}>
        {recipe.map((pickup, idx) => (
          <PickupImage key={idx} pickup={Pickup[pickup] as unknown as Pickup} />
        ))}
      </div>
    </div>
  )
}