import { Component, EventEmitter, Input, OnInit, Output } from '@angular/core';
import { Observable } from 'rxjs';
import { RadioButtonProps } from './RadioButtonProps';

@Component({
  selector: 'app-radio-button',
  standalone: true,
  imports: [],
  templateUrl: './radio-button.component.html',
  styleUrl: './radio-button.component.css'
})
export class RadioButtonComponent {
  @Input() props: RadioButtonProps = {
    text:"", onToggle:()=>{}, isChecked:false
  };
  @Output() onToggle = new EventEmitter<boolean>();

  toggleRadio() {
    this.props.isChecked = !this.props.isChecked; // Toggle the checked state
    this.props.onToggle(this.props.isChecked);
    this.onToggle.emit(this.props.isChecked)
  }

}
