import { CommonModule } from '@angular/common';
import { Component, Input } from '@angular/core';
import { MatIconModule } from '@angular/material/icon';

@Component({
  selector: 'app-button-w-icon',
  standalone: true,
  imports: [CommonModule, MatIconModule],
  templateUrl: './button-w-icon.component.html',
  styleUrl: './button-w-icon.component.scss'
})
export class ButtonWIconComponent {
  @Input() iconName = "";
  @Input() text = "";
}
