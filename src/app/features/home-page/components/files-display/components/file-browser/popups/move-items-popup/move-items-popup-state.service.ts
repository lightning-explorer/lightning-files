import { Injectable } from "@angular/core";
import { PersistentConfigService } from "@core/services/persistence/config.service";
import { BehaviorSubject } from "rxjs";

@Injectable()
export class MoveItemsPopupStateService {
  private isVisibleSubject = new BehaviorSubject<boolean>(false);
  isVisible$ = this.isVisibleSubject.asObservable();

  constructor(private configService: PersistentConfigService) {}

  async getDontAskAgain() {
    return await this.configService.readOrSet("moveItemsDontAskAgain", false);
  }
  async setDontAskAgain(val: boolean) {
    this.configService.update("moveItemsDontAskAgain", val);
  }

  async attemptOpen(): Promise<boolean> {
    const dontAskAgain = await this.getDontAskAgain();
    if (!dontAskAgain) {
      this.isVisibleSubject.next(true);
      return true;
    }
    return false;
  }

  async closePopup() {
    this.isVisibleSubject.next(false);
  }
}
