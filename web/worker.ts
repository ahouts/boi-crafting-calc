import init, { CraftingCache } from '../pkg/boi_crafting_calc'
import {
  visit_request,
  WorkerRequest,
  worker_response_craft,
  worker_response_ready,
  WorkerRequestVisitor,
  WorkerRequestCraft,
} from './api'

const worker: DedicatedWorkerGlobalScope = self
const db_name = 'crafting-cache'
const db_version = 1
const store_name = 'crafting-cache-store'
const object_key = 'crafting-cache-item'

async function setup_database(): Promise<IDBDatabase> {
  const factory = worker.indexedDB.open(db_name, db_version)
  factory.onupgradeneeded = (message) => {
    const db: IDBDatabase = factory.result
    if (message.oldVersion < 1) {
      db.createObjectStore(store_name)
    }
  }
  await new Promise((resolve, reject) => {
    factory.onsuccess = resolve
    factory.onerror = reject
  })
  return factory.result
}

async function get_cache_if_exists(db: IDBDatabase): Promise<CraftingCache | null> {
  const tx = db.transaction(store_name, 'readonly')
  const store = tx.objectStore(store_name)
  const res = store.get(object_key)
  await new Promise((resolve, reject) => {
    res.onsuccess = resolve
    res.onerror = reject
  })
  const result: Uint8Array | undefined = res.result
  if (result === undefined) {
    return null
  }
  return CraftingCache.deserialize(result)
}

async function save_cache(db: IDBDatabase, cache: CraftingCache) {
  const tx = db.transaction(store_name, 'readwrite')
  const store = tx.objectStore(store_name)
  const res = store.put(cache.serialize(), object_key)
  await new Promise((resolve, reject) => {
    res.onsuccess = resolve
    res.onerror = reject
  })
}

;(async () => {
  await init()

  const db = await setup_database()
  const maybe_cache = await get_cache_if_exists(db)
  let cache: CraftingCache
  if (maybe_cache === null) {
    cache = new CraftingCache()
    await save_cache(db, cache)
  } else {
    cache = maybe_cache
  }

  worker.onmessage = (message) => {
    const msg: WorkerRequest = message.data
    visit_request(
      msg,
      new (class implements WorkerRequestVisitor<void> {
        visit_craft(craft: WorkerRequestCraft) {
          const item_id = cache.craft(craft.pickups)
          worker.postMessage(
            worker_response_craft({
              request_id: craft.request_id,
              item_id,
            }),
          )
        }

        visit_shutdown(): void {
          cache.free()
          worker.close()
        }
      })(),
    )
  }

  worker.postMessage(worker_response_ready)
})()
