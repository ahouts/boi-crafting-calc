import crafting_pointer from './available_cache'
import { Pickup } from '../pkg'

// const worker = new Worker(new URL('./worker.ts', import.meta.url), {
//   type: 'module',
// })
// worker.onerror = console.error;
// worker.onmessage = console.log;

(async () => {
  const ptr = await crafting_pointer;
  console.log(ptr
    .get()
    .craft([
      Pickup.Key,
      Pickup.LuckyPenny,
      Pickup.Nickel,
      Pickup.RedHeart,
      Pickup.RedHeart,
      Pickup.Penny,
      Pickup.Penny,
      Pickup.SoulHeart,
    ]));
})();