function assertUnreachable(_: never): never {
    throw new Error("unreachable code reached...");
}

type WorkerRequestType = 'shutdown';

interface WorkerRequest {
    type: WorkerRequestType;
}

class WorkerRequestShutdown implements WorkerRequest {
    type: 'shutdown'
}

type WorkerResponseType = 'ready' | 'answer';

interface WorkerResponse {
    type: WorkerResponseType,
}

class WorkerResponseReady implements WorkerResponse {
    type: 'ready'
}

class WorkerResponseAnswer implements WorkerResponse {
    type: 'answer'
}
