import init, { BasicCrafter, ItemId, Pickup } from '../pkg/boi_crafting_calc'

export interface Crafter {
  priority: number

  craft(pickups: Array<Pickup>): Promise<ItemId>

  shutdown(): void
}

class CrafterPointer {
  private crafter: Crafter | null

  constructor() {
    this.crafter = null
  }

  public get(): Crafter | null {
    return this.crafter
  }

  update(new_crafter: Crafter) {
    if (this.crafter === null || this.crafter.priority < new_crafter.priority) {
      this.clear()
      this.crafter = new_crafter
    }
  }

  clear() {
    if (this.crafter !== null) {
      this.crafter.shutdown()
      this.crafter = null
    }
  }
}

const pointer = new CrafterPointer()

export default (async (): Promise<CrafterPointer> => {
  await init()

  pointer.update(
    new (class implements Crafter {
      readonly priority: number
      readonly crafter: BasicCrafter

      constructor() {
        this.priority = 1
        this.crafter = new BasicCrafter()
      }

      craft(pickups: Array<Pickup>): Promise<ItemId> {
        return Promise.resolve(this.crafter.craft(pickups))
      }

      shutdown() {
        this.crafter.free()
      }
    })(),
  )

  return pointer
})()

export function update_crafter(crafter: Crafter): void {
  pointer.update(crafter)
}
