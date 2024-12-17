import { CommonModule } from '@angular/common';
import { Component, Input } from '@angular/core';
import { ModalPopupComponent } from "@shared/components/popups/modal-popup/modal-popup.component";
import { ButtonModel } from '@shared/components/popups/modal-popup/models/ButtonModel';
import { RadioButtonComponent } from "@shared/components/buttons/radio-button/radio-button.component";
import { PersistentConfigService } from '@core/services/persistence/config.service';
import { RadioButtonProps } from '@shared/components/buttons/radio-button/RadioButtonProps';

@Component({
  selector: 'app-move-items-popup',
  standalone: true,
  imports: [CommonModule, ModalPopupComponent, RadioButtonComponent],
  templateUrl: './move-items-popup.component.html',
  styleUrl: './move-items-popup.component.css'
})
export class MoveItemsPopupComponent {
  isVisible = false;
  itemsAdding = 0;
  pathFrom = "test";
  destPath = "test";

  private get dontAskAgain(): boolean {
    return this.config.read("moveItemsDontAskAgain");
  }
  private set dontAskAgain(val: boolean) {
    this.config.update("moveItemsDontAskAgain", val);
  }

  private onYesCallback: (() => void) | undefined;

  constructor(private config: PersistentConfigService) { }

  buttons: ButtonModel[] = [{ text: "Yes", action: () => this.onYesClicked() }];

  dontAskAgainRadioButton: RadioButtonProps = {
    text: "Don't ask again",
    onToggle: (val: boolean) => val,
    isChecked: this.dontAskAgain // This now dynamically reflects the state of dontAskAgain
  };


  open(pathFrom: string, destPath: string, itemsAdding: number, onYesCallback: () => void) {
    if (!this.dontAskAgain) {
      this.isVisible = true;
      this.itemsAdding = itemsAdding;
      this.pathFrom = pathFrom;
      this.destPath = destPath;
      this.onYesCallback = onYesCallback;
    }
  }

  onYesClicked() {
    if (this.onYesCallback)
      this.onYesCallback();
  }
}
