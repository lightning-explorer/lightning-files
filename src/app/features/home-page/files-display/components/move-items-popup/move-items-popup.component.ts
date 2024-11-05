import { CommonModule } from '@angular/common';
import { Component, Input } from '@angular/core';
import { ModalPopupComponent } from "../../../../../shared/components/modal-popup/modal-popup.component";
import { ButtonModel } from '../../../../../shared/components/modal-popup/models/ButtonModel';

@Component({
  selector: 'app-move-items-popup',
  standalone: true,
  imports: [CommonModule, ModalPopupComponent],
  templateUrl: './move-items-popup.component.html',
  styleUrl: './move-items-popup.component.css'
})
export class MoveItemsPopupComponent {
  isVisible = false;
  itemsAdding = 0;
  pathFrom = "test";
  destPath = "test";
  private onYesCallback: (() => void) | undefined;

  buttons: ButtonModel[] = [{ text: "Yes", action: () => this.onYesClicked() }];

  open(pathFrom: string, destPath: string, itemsAdding: number, onYesCallback: () => void) {
    this.isVisible = true;
    this.itemsAdding = itemsAdding;
    this.pathFrom = pathFrom;
    this.destPath = destPath;
    this.onYesCallback = onYesCallback;
  }

  onYesClicked() {
    if (this.onYesCallback)
      this.onYesCallback();
  }
}
