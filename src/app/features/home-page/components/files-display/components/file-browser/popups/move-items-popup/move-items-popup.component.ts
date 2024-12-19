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

  @Input() isVisible: boolean = false;
  @Input() itemsAdding: number = 0;
  @Input() pathFrom: string = '';
  @Input() destPath: string = '';

  private get dontAskAgain(): boolean {
    return this.config.read("moveItemsDontAskAgain");
  }
  private set dontAskAgain(val: boolean) {
    this.config.update("moveItemsDontAskAgain", val);
  }

  constructor(private config: PersistentConfigService) { }

  buttons: ButtonModel[] = [{ text: "Yes", action: () => this.onYesClicked() }];

  dontAskAgainRadioButton: RadioButtonProps = {
    text: "Don't ask again",
    onToggle: (val: boolean) => val,
    isChecked: this.dontAskAgain // This now dynamically reflects the state of dontAskAgain
  };


  onYesClicked() {
    if (this.onYesCallBack){
      this.props.onYesCallBack()
      this.destroy();
    }
  }

  private destroy(){
    if(this.props?.onDestroy)
      this.props.onDestroy();
  }
}
