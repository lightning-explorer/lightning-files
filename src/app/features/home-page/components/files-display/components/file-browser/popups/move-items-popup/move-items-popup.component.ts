import { CommonModule } from "@angular/common";
import { Component, Input, OnDestroy, OnInit } from "@angular/core";
import { ModalPopupComponent } from "@shared/components/popups/modal-popup/modal-popup.component";
import { ButtonModel } from "@shared/components/popups/modal-popup/models/ButtonModel";
import { RadioButtonComponent } from "@shared/components/buttons/radio-button/radio-button.component";
import { PersistentConfigService } from "@core/services/persistence/config.service";
import { RadioButtonProps } from "@shared/components/buttons/radio-button/RadioButtonProps";
import { SelectService } from "../../services/interaction/select.service";
import { DragDropService } from "../../services/interaction/dragdrop.service";
import { Subscription } from "rxjs";
import { DirectoryNavigatorService } from "src/app/features/home-page/services/directory-navigator.service";

@Component({
  selector: "app-move-items-popup",
  standalone: true,
  imports: [CommonModule, ModalPopupComponent, RadioButtonComponent],
  templateUrl: "./move-items-popup.component.html",
  styleUrl: "./move-items-popup.component.css",
})
export class MoveItemsPopupComponent implements OnInit, OnDestroy {
  private subscription = new Subscription();

  currentDir$ = this.directoryNavService.currentDir$;
  itemsAdding$ = this.selectService.selectedIndices$;
  dest$ = this.dragDropService.draggingItemsTo$;
  _isVisible = false;

  constructor(
    private config: PersistentConfigService,
    private directoryNavService: DirectoryNavigatorService,
    private selectService: SelectService,
    private dragDropService: DragDropService
  ) {}

  private async getDontAskAgain() {
    return await this.config.readOrSet("moveItemsDontAskAgain", false);
  }
  private async setDontAskAgain(val: boolean) {
    this.config.update("moveItemsDontAskAgain", val);
  }

  buttons: ButtonModel[] = [{ text: "Yes", action: () => this.onYesClicked() }];

  dontAskAgainRadioButton: RadioButtonProps = {
    text: "Don't ask again",
    onToggle: (val: boolean) => val,
    isChecked: false /*this.getDontAskAgain*/, // This now dynamically reflects the state of dontAskAgain
  };

  /** Returns `false` if the config has disabled this popup */
  async attemptOpen() {
    if (!this.getDontAskAgain) {
      this._isVisible = true;
      return true;
    }
    return false;
  }

  async onYesClicked() {
    await this.dragDropService.moveDraggedItemsAsync();
  }

  onDestroy() {
    this.dragDropService.unhideAllDraggingItems();
  }

  ngOnInit(): void {}

  ngOnDestroy(): void {
    this.subscription.unsubscribe();
  }
}
