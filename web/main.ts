import crafting_pointer, { update_crafter } from './available_crafter'
import { Pickup } from '../pkg'
import { WorkerApi } from './worker_api'

const worker = new Worker(new URL('./worker.ts', import.meta.url), {
  type: 'module',
})
worker.onerror = console.error
;(async () => {
  new WorkerApi(worker, update_crafter)
  const ptr = await crafting_pointer

  const run_craft = async () => {
    const crafter = ptr.get()
    console.log(
      crafter === null
        ? 'pointer is null'
        : await crafter.craft([
            Pickup.Key,
            Pickup.LuckyPenny,
            Pickup.Nickel,
            Pickup.RedHeart,
            Pickup.RedHeart,
            Pickup.Penny,
            Pickup.Penny,
            Pickup.SoulHeart,
          ]),
    )
    setTimeout(run_craft, 5000)
  }

  setTimeout(() => {
    ptr.clear()
  }, 50000)

  await run_craft()
})()
