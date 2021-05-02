import {
  RequestId,
  visit_response,
  worker_request_craft,
  WorkerResponse,
  WorkerResponseCraft,
  WorkerResponseVisitor,
} from './api'
import { Pickup, ItemId } from '../pkg'
import { Crafter } from './available_crafter'

export class WorkerApi implements Crafter {
  private readonly worker: Worker
  private request_id: RequestId
  private readonly ready_handler: (api: WorkerApi) => void
  private readonly in_flight_craft_requests: Record<RequestId, (arg: ItemId) => void>

  readonly priority: number

  constructor(worker: Worker, ready_handler: (api: WorkerApi) => void) {
    this.worker = worker
    this.request_id = 1
    this.ready_handler = ready_handler
    this.in_flight_craft_requests = {}
    this.priority = 2
    const self = this
    worker.onmessage = (message: MessageEvent<WorkerResponse>) => {
      visit_response(
        message.data,
        new (class implements WorkerResponseVisitor<void> {
          visit_craft(craft: WorkerResponseCraft) {
            self.in_flight_craft_requests[craft.request_id](craft.item_id)
            delete self.in_flight_craft_requests[craft.request_id]
          }
          visit_ready(): void {
            self.ready_handler(self)
          }
        })(),
      )
    }
  }

  async craft(pickups: Array<Pickup>): Promise<ItemId> {
    this.worker.postMessage(
      worker_request_craft({
        request_id: this.request_id,
        pickups,
      }),
    )
    const promise = new Promise<ItemId>((resolve) => {
      this.in_flight_craft_requests[this.request_id] = resolve
    })
    this.request_id += 1
    return await promise
  }
}
