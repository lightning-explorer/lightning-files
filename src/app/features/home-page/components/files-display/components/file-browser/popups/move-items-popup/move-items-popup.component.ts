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

@Component({
  selector: "app-move-items-popup",
  standalone: true,
  imports: [CommonModule, ModalPopupComponent, RadioButtonComponent],
  templateUrl: "./move-items-popup.component.html",
  styleUrl: "./move-items-popup.component.css",
})
export class MoveItemsPopupComponent implements OnInit, OnDestroy {
  private subscription = new Subscription();

  itemsAdding$ = this.selectService.selectedIndices$;
  destPath$ = this.dragDropService.draggingItemsTo$;

  @Input() isVisible = false;
  @Input() pathFrom = "";
  @Input() onYesClicked = () => {};
  @Input() onDestroy = () => {};

  constructor(
    private config: PersistentConfigService,
    private selectService: SelectService,
    private dragDropService: DragDropService
  ) {}

  private get dontAskAgain(): boolean {
    return this.config.read("moveItemsDontAskAgain");
  }
  private set dontAskAgain(val: boolean) {
    this.config.update("moveItemsDontAskAgain", val);
  }

  buttons: ButtonModel[] = [{ text: "Yes", action: () => this.onYesClicked() }];

  dontAskAgainRadioButton: RadioButtonProps = {
    text: "Don't ask again",
    onToggle: (val: boolean) => val,
    isChecked: this.dontAskAgain, // This now dynamically reflects the state of dontAskAgain
  };

  ngOnInit(): void {}

  ngOnDestroy(): void {
    this.subscription.unsubscribe();
  }
}
