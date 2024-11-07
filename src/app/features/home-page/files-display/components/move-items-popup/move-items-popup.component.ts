import { CommonModule } from '@angular/common';
import { Component, Input } from '@angular/core';
import { ModalPopupComponent } from "../../../../../shared/components/modal-popup/modal-popup.component";
import { ButtonModel } from '../../../../../shared/components/modal-popup/models/ButtonModel';
import { RadioButtonComponent } from "../../../../../shared/components/radio-button/radio-button.component";
import { RadioButtonModel } from '../../../../../shared/components/modal-popup/models/RadioButtonModel';
import { PersistentConfigService } from '../../../../../core/services/persistence/config.service';
import { BehaviorSubject } from 'rxjs';

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

  private get dontAskAgain():boolean{
    return this.config.read("moveItemsDontAskAgain");
  }
  private set dontAskAgain(val:boolean){
    this.config.update("moveItemsDontAskAgain", val);
  }

  private onYesCallback: (() => void) | undefined;

  constructor(private config: PersistentConfigService) { }

  buttons: ButtonModel[] = [{ text: "Yes", action: () => this.onYesClicked() }];

  get radioButtons(): RadioButtonModel[] {
    return [
      {
        text: "Don't ask again",
        onToggle: (val) => this.dontAskAgain = val,
        isChecked: this.dontAskAgain // This now dynamically reflects the state of dontAskAgain
      }
    ];
  }

  async open(pathFrom: string, destPath: string, itemsAdding: number, onYesCallback: () => void) {
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
