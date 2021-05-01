import init, {CraftingCache} from '../pkg/boi_crafting_calc';
import {assertUnreachable, WorkerRequest, WorkerResponseReady} from './api';

const worker: DedicatedWorkerGlobalScope = self;
const dbName = 'crafting-cache';
const dbVersion = 1;
const storeName = 'crafting-cache-store';
const objectKey = 'crafting-cache-item';

async function setupDatabase(): Promise<IDBDatabase> {
    const factory = worker.indexedDB.open(dbName, dbVersion);
    factory.onupgradeneeded = message => {
        const db: IDBDatabase = factory.result;
        if (message.oldVersion < 1) {
            db.createObjectStore(storeName);
        }
    };
    await new Promise((resolve, reject) => {
        factory.onsuccess = resolve;
        factory.onerror = reject;
    });
    return factory.result;
}

async function getCacheIfExists(db: IDBDatabase): Promise<CraftingCache | null> {
    const tx = db.transaction(storeName, 'readonly');
    const store = tx.objectStore(storeName);
    const res = store.get(objectKey);
    await new Promise((resolve, reject) => {
        res.onsuccess = resolve;
        res.onerror = reject;
    });
    const result: Uint8Array | undefined = res.result;
    if (result === undefined) {
        return null;
    }
    return CraftingCache.deserialize(result);
}

async function saveCache(db: IDBDatabase, cache: CraftingCache) {
    const tx = db.transaction(storeName, 'readwrite');
    const store = tx.objectStore(storeName);
    const res = store.put(cache.serialize(), objectKey);
    await new Promise((resolve, reject) => {
        res.onsuccess = resolve;
        res.onerror = reject;
    });
}

(async () => {
    await init();

    const db = await setupDatabase();
    let cache = await getCacheIfExists(db);
    if (cache === null) {
        cache = CraftingCache.new();
        await saveCache(db, cache);
    }

    worker.onmessage = (message) => {
        const msg: WorkerRequest = message.data;
        switch (msg.type) {
            case 'shutdown':
                cache.free();
                worker.close();
                break;
            default:
                assertUnreachable(msg.type);
        }
    };

    worker.postMessage(WorkerResponseReady);
})();
