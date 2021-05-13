import * as React from 'react'
import { useDispatch, useSelector } from 'react-redux'
import { clear_recipes, select_crafter } from '../slices/crafter_slice'
import { Recipe } from './recipe'
import { Item } from './item'
import { string_or_empty } from '../util/defaults'
import { FaWindowClose } from 'react-icons/fa'

export type RecipesParams = {
  classes?: string,
}

export function Recipes(params: RecipesParams = {}): React.ReactElement {
  const recipes = useSelector(select_crafter).recipes
  const dispatch = useDispatch()
  return recipes === null ? (
    <div className={`${string_or_empty(params.classes)}`} />
  ) : (
    <div className={`bg-gray-700 ${string_or_empty(params.classes)}`}>
      <div className={'flex'}>
        <Item item_id={recipes.item_id} />
        <div className={'flex-grow'} />
        <FaWindowClose className={'object-contain w-10 h-10'} onClick={() => dispatch(clear_recipes())} />
      </div>
      <p>Item Id: {recipes.item_id}</p>
      {recipes.recipes.map((recipe, idx) => (
        <Recipe key={idx} recipe={recipe} />
      ))}
    </div>
  )
}
