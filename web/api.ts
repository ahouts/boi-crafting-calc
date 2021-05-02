import { Pickup, ItemId } from '../pkg'

function assert_unreachable(v: never): never {
  throw new Error(`unreachable code reached: ${v}`)
}

type OmitType<T> = Omit<T, 'type'>

type ApiBuilder<T> = (opts: OmitType<T>) => T

export type WorkerRequestType = 'craft'

export interface WorkerRequest {
  type: WorkerRequestType
}

export interface WorkerRequestVisitor<T> {
  visit_craft(craft: WorkerRequestCraft): T
}

export function visit_request<T>(request: WorkerRequest, visitor: WorkerRequestVisitor<T>): T {
  switch (request.type) {
    case 'craft':
      return visitor.visit_craft(request as WorkerRequestCraft)
    default:
      assert_unreachable(request.type)
  }
}

export type RequestId = number

export type WorkerRequestCraft = {
  type: 'craft'
  request_id: RequestId
  pickups: Array<Pickup>
}

export const worker_request_craft: ApiBuilder<WorkerRequestCraft> = (opts) => ({
  type: 'craft',
  ...opts,
})

export type WorkerResponseType = 'ready' | 'craft'

export interface WorkerResponse {
  type: WorkerResponseType
}

export interface WorkerResponseVisitor<T> {
  visit_ready(): T

  visit_craft(craft: WorkerResponseCraft): T
}

export function visit_response<T>(response: WorkerResponse, visitor: WorkerResponseVisitor<T>): T {
  switch (response.type) {
    case 'ready':
      return visitor.visit_ready()
    case 'craft':
      return visitor.visit_craft(response as WorkerResponseCraft)
    default:
      assert_unreachable(response.type)
  }
}

export const worker_response_ready: WorkerResponse = {
  type: 'ready',
}

export type WorkerResponseCraft = {
  type: 'craft'
  request_id: RequestId
  item_id: ItemId
}

export const worker_response_craft: ApiBuilder<WorkerResponseCraft> = (opts) => ({
  type: 'craft',
  ...opts,
})
