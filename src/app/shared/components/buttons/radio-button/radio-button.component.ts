import { Component, EventEmitter, Input, OnInit, Output } from '@angular/core';
import { Observable } from 'rxjs';

@Component({
  selector: 'app-radio-button',
  standalone: true,
  imports: [],
  templateUrl: './radio-button.component.html',
  styleUrl: './radio-button.component.css'
})
export class RadioButtonComponent {
  @Input() isChecked:boolean = false;
  @Output() onToggle = new EventEmitter<boolean>();

  toggleRadio() {
    this.isChecked = !this.isChecked; // Toggle the checked state
    this.onToggle.emit(this.isChecked)
  }

}
