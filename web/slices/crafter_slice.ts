import { createSlice, Draft, PayloadAction } from '@reduxjs/toolkit'
import { DeltaCrafter, ItemId, Pickup, Recipe } from '../../pkg'
import { RootState } from '../store'
import { Selector } from 'react-redux'

function update_fields(state: Draft<CrafterState>) {
  if (state.crafter !== null) {
    state.items = state.crafter.items()
    state.pickups = Object.fromEntries(
      Object.entries(state.crafter.pickups())
        .map(([ k, v ]) => [ Pickup[k as unknown as number], v ]),
    )
  }
}

function native_pickup(pickup: Pickup): Pickup {
  return isNaN(Number(pickup)) ?
    Pickup[pickup] as unknown as Pickup :
    pickup
}

export const crafter_slice = createSlice({
  name: 'crafter',
  initialState: {
    crafter: null as DeltaCrafter | null,
    pickups: {} as Record<Pickup, number>,
    items: [] as Array<ItemId>,
    recipes: null as { item_id: ItemId, recipes: Array<Recipe> } | null,
  },
  reducers: {
    set: (state, payload: PayloadAction<DeltaCrafter>) => {
      state.crafter = payload.payload
    },
    clear: (state) => {
      if (state.crafter !== null) {
        state.crafter.free()
        state.crafter = null
      }
    },
    add_pickup: (state, payload: PayloadAction<Pickup>) => {
      if (state.crafter !== null) {
        state.crafter.add_pickup(native_pickup(payload.payload))
        update_fields(state)
      }
    },
    remove_pickup: (state, payload: PayloadAction<Pickup>) => {
      if (state.crafter !== null) {
        state.crafter.remove_pickup(native_pickup(payload.payload))
        update_fields(state)
      }
    },
    reset: (state) => {
      if (state.crafter !== null) {
        state.crafter.reset()
        update_fields(state)
      }
    },
    get_recipes: (state, payload: PayloadAction<ItemId>) => {
      if (state.crafter !== null) {
        state.recipes = {
          item_id: payload.payload,
          recipes: state.crafter.get_recipes(payload.payload),
        }
      }
    },
  },
})

export type CrafterState = ReturnType<typeof crafter_slice.reducer>
export const { clear, set, add_pickup, remove_pickup, reset, get_recipes } = crafter_slice.actions
export const select_crafter: Selector<RootState, CrafterState> = store => store.crafter
