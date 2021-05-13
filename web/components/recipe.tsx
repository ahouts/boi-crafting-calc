import * as React from 'react'
import { Pickup, Recipe as TRecipe } from '../../pkg'
import { PickupImage } from './pickup/pickup_image'

export type RecipeParams = {
  recipe: TRecipe,
}

export function Recipe({ recipe }: RecipeParams): React.ReactElement {
  return (
    <div className={'grid grid-cols-4 auto-rows-auto bg-gray-800'}>
      {recipe.map((pickup, idx) => (
        <PickupImage key={idx} pickup={Pickup[pickup] as unknown as Pickup} />
      ))}
    </div>
  )
}