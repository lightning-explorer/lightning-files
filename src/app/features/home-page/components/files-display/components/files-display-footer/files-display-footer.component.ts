import { Component } from '@angular/core';
import { IconifyIconModule } from "../../../../../../shared/components/icons/IconifyIcons/icon.module";
import { HideOverflowDirective } from './hide-overflow.directive';
import { SearchOverlayStateService } from '../search-overlay/services/search-overlay-state.service';

@Component({
  selector: 'app-files-display-footer',
  standalone: true,
  imports: [IconifyIconModule, HideOverflowDirective],
  templateUrl: './files-display-footer.component.html',
  styleUrl: './files-display-footer.component.css'
})
export class FilesDisplayFooterComponent {
  constructor(private searchStateService:SearchOverlayStateService){}


  onSearchClick(){
    this.searchStateService.setVisibility(true);
  }
}
