export function assertUnreachable(_: never): never {
    throw new Error("unreachable code reached...");
}

export type WorkerRequestType = 'shutdown';

export interface WorkerRequest {
    type: WorkerRequestType;
}

export const WorkerRequestShutdown: WorkerRequest = {
    type: 'shutdown'
}

export type WorkerResponseType = 'ready' | 'answer'

export interface WorkerResponse {
    type: WorkerResponseType,
}

export const WorkerResponseReady: WorkerResponse = {
    type: "ready",
}

export const WorkerResponseAnswer: WorkerResponse = {
    type: 'answer'
}
