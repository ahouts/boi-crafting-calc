async function main() {
    const worker = new Worker(new URL('./worker.ts', import.meta.url), {type: 'module'});
    worker.onerror = console.error;
    worker.onmessage = console.log;
}

main();