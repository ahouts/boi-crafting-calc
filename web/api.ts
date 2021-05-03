import { Pickup, ItemId } from '../pkg'

function assert_unreachable(v: never): never {
  throw new Error(`unreachable code reached: ${v}`)
}

type OmitType<T> = Omit<T, 'type'>

type ApiBuilder<T> = (opts: OmitType<T>) => T

export type WorkerRequestType = 'craft' | 'possible_items' | 'shutdown'

export interface WorkerRequest {
  type: WorkerRequestType
}

export interface WorkerRequestVisitor<T> {
  visit_craft(craft: WorkerRequestCraft): T

  visit_possible_items(possible_items: WorkerRequestPossibleItems): T

  visit_shutdown(): T
}

export function visit_request<T>(request: WorkerRequest, visitor: WorkerRequestVisitor<T>): T {
  switch (request.type) {
    case 'craft':
      return visitor.visit_craft(request as WorkerRequestCraft)
    case 'possible_items':
      return visitor.visit_possible_items(request as WorkerRequestPossibleItems)
    case 'shutdown':
      return visitor.visit_shutdown()
    default:
      assert_unreachable(request.type)
  }
}

export type RequestId = number

export type WorkerRequestCraft = WorkerRequest & {
  request_id: RequestId
  pickups: Array<Pickup>
}

export const worker_request_craft: ApiBuilder<WorkerRequestCraft> = (opts) => ({
  type: 'craft',
  ...opts,
})

export type WorkerRequestPossibleItems = WorkerRequest & {
  request_id: RequestId
  pickups: Array<Pickup>
}

export const worker_request_possible_items: ApiBuilder<WorkerRequestPossibleItems> = (opts) => ({
  type: 'possible_items',
  ...opts,
})

export const worker_request_shutdown: WorkerRequest = {
  type: 'shutdown',
}

export type WorkerResponseType = 'ready' | 'craft' | 'possible_items'

export interface WorkerResponse {
  type: WorkerResponseType
}

export interface WorkerResponseVisitor<T> {
  visit_ready(): T

  visit_craft(craft: WorkerResponseCraft): T

  visit_possible_items(possible_items: WorkerResponsePossibleItems): T
}

export function visit_response<T>(response: WorkerResponse, visitor: WorkerResponseVisitor<T>): T {
  switch (response.type) {
    case 'ready':
      return visitor.visit_ready()
    case 'craft':
      return visitor.visit_craft(response as WorkerResponseCraft)
    case 'possible_items':
      return visitor.visit_possible_items(response as WorkerResponsePossibleItems)
    default:
      assert_unreachable(response.type)
  }
}

export const worker_response_ready: WorkerResponse = {
  type: 'ready',
}

export type WorkerResponseCraft = WorkerResponse & {
  request_id: RequestId
  item_id: ItemId
}

export const worker_response_craft: ApiBuilder<WorkerResponseCraft> = (opts) => ({
  type: 'craft',
  ...opts,
})

export type WorkerResponsePossibleItems = WorkerResponse & {
  request_id: RequestId
  item_ids: Array<ItemId>
}

export const worker_response_possible_items: ApiBuilder<WorkerResponsePossibleItems> = (opts) => ({
  type: 'possible_items',
  ...opts,
})
