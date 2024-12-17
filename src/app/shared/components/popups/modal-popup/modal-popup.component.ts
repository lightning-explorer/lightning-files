import { CommonModule } from '@angular/common';
import { Component, EventEmitter, Input, Output } from '@angular/core';
import { ButtonModel } from './models/ButtonModel';

@Component({
  selector: 'app-modal-popup',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './modal-popup.component.html',
  styleUrl: './modal-popup.component.css'
})
export class ModalPopupComponent {
  @Input() title: string = 'Modal Title';
  @Input() isVisible: boolean = false;
  @Input() buttons: ButtonModel[] = [];
  @Output() isVisibleChange = new EventEmitter<boolean>();

  close() {
    this.isVisible = false;
    this.isVisibleChange.emit(this.isVisible);
  }

  open() {
    this.isVisible = true;
    this.isVisibleChange.emit(this.isVisible);
  }

  onButtonClick(action: () => void) {
    action();
    this.close();
  }
}
