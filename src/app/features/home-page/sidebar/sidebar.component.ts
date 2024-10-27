import { Component } from '@angular/core';
import { ButtonWIconComponent } from "../../../shared/button-w-icon/button-w-icon.component";

@Component({
  selector: 'app-sidebar',
  standalone: true,
  imports: [ButtonWIconComponent],
  templateUrl: './sidebar.component.html',
  styleUrl: './sidebar.component.scss'
})
export class SidebarComponent {

}
