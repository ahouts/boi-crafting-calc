import init, { BasicCrafter, Crafter } from '../pkg/boi_crafting_calc'

class Pointer<T> {
  private val: T

  constructor(val: T) {
    this.val = val
  }

  public get(): T {
    return this.val
  }

  update(newVal: T) {
    this.val = newVal
  }
}

export default (async (): Promise<Pointer<Crafter>> => {
  await init()

  const pointer: Pointer<Crafter> = new Pointer(new BasicCrafter())

  return pointer
})()
