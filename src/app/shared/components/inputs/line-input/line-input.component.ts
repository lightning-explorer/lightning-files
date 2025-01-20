import { Component, ElementRef, EventEmitter, Input, Output, ViewChild } from '@angular/core';
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-line-input',
  standalone: true,
  imports: [FormsModule],
  templateUrl: './line-input.component.html',
  styleUrl: './line-input.component.css'
})
export class LineInputComponent {
  @ViewChild("input") input!:ElementRef;
  @Input() text = "";
  @Output() textChanged = new EventEmitter<string>();

  onInputChange(event:any): void {
    this.textChanged.emit(event.value);
  }

  focus(){
    this.input.nativeElement.focus();
  }
}
