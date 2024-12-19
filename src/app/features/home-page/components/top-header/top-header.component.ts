import { Component } from '@angular/core';
import { CurrentDirectoryBarComponent } from "./current-directory-bar/current-directory-bar.component";
import { MatIconModule } from '@angular/material/icon';
import { DirectoryNavigatorService } from '@core/services/files/directory-navigator/directory-navigator.service';
import { SearchbarComponent } from './searchbar/searchbar.component';
import { DirectoryHistoryService } from '@core/services/files/directory-navigator/directory-history.service';

@Component({
  selector: 'app-top-header',
  standalone: true,
  imports: [CurrentDirectoryBarComponent, SearchbarComponent, MatIconModule],
  templateUrl: './top-header.component.html',
  styleUrl: './top-header.component.css'
})
export class TopHeaderComponent {

  constructor(private directoryService: DirectoryNavigatorService, 
    private directoryHistoryService:DirectoryHistoryService) { }

  async onNavigateBackDirectoryClick() {
    let parent = await this.directoryService.getParentDirectory()
    await this.directoryService.setCurrentDir(parent);
  }

  async onUndoClick(){
    this.directoryHistoryService.undo();
  }

  async onRedoClick(){
    this.directoryHistoryService.redo();
  }

}
