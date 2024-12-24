import { CommonModule } from '@angular/common';
import { Component, Input } from '@angular/core';
import { QuickAccessPath } from '@core/services/files/quick-access.service';
import { ButtonWIconComponent } from "../../../../shared/components/buttons/button-w-icon/button-w-icon.component";

@Component({
  selector: 'app-quick-access-shortcut',
  standalone: true,
  imports: [CommonModule, ButtonWIconComponent],
  templateUrl: './quick-access-shortcut.component.html',
  styleUrl: './quick-access-shortcut.component.css'
})
export class QuickAccessShortcutComponent {
  @Input() path:QuickAccessPath | undefined;

  constructor(){}
}
