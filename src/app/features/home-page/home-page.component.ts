import { Component, HostListener, OnInit } from '@angular/core';
import { SearchEngineService } from '../../core/services/search/search-engine.service';
import { SearchbarComponent } from "./searchbar/searchbar.component";
import { FileDTOReceived } from '../../core/dtos/file-dto-received';
import { FileResultComponent } from "./file-result/file-result.component";
import { CommonModule } from '@angular/common';
import { FormControl } from '@angular/forms';
import { SearchParamsDTO } from '../../core/dtos/search-params-dto';
import { SidebarComponent } from "./sidebar/sidebar.component";
import { CurrentDirectoryBarComponent } from "./current-directory-bar/current-directory-bar.component";
import { FilesDisplayComponent } from "./files-display/files-display.component";
import { DirectoryNavigatorService } from '../../core/services/files/directory-navigator.service';
import { MatIconModule } from '@angular/material/icon';
import { InlineQueryDTO } from '../../core/dtos/inline-query-dto';
import { InlineSearchService } from '../../core/services/search/inline-search.service';

// TODO:
// put search bar in Shared and then make a simpler one in features to manage its own state

@Component({
  selector: 'app-home-page',
  standalone: true,
  imports: [SearchbarComponent, FileResultComponent, CommonModule, SidebarComponent, CurrentDirectoryBarComponent, FilesDisplayComponent, MatIconModule],
  templateUrl: './home-page.component.html',
  styleUrl: './home-page.component.scss'
})
export class HomePageComponent implements OnInit {

  inputControl = new FormControl();

  searchResults: FileDTOReceived[] = [];
  driveFiles: FileDTOReceived[] = [];

  constructor(private searchEngineService: SearchEngineService,
    private directoryService: DirectoryNavigatorService,
    private inlineSearchService: InlineSearchService
  ) { }

  ngOnInit(): void {
    this.directoryService.setDriveFiles();

    this.directoryService.currentFiles$.subscribe(x =>
      this.driveFiles = x
    );
  }

  async search(value: string) {
    let searchParams: SearchParamsDTO = {
      FilePath: value
    }
    let results = await this.searchEngineService.query(searchParams);
    this.searchResults = results;
  }

  searchBarBlur() {
    setTimeout(() => {
      this.searchResults.length = 0;
    }, 100)

  }

  async onNavigateBackDirectoryClick() {
    let parent = await this.directoryService.getParentDirectory()
    await this.directoryService.setCurrentDir(parent);
  }

  searchQuery = "";
  @HostListener('window:keydown', ['$event'])
  async handleKeydown(event: KeyboardEvent) {

    if (!this.isInputFocused()) {
      this.searchQuery += event.key;
      const queryDto: InlineQueryDTO = { Query: this.searchQuery }
      const dtos = await this.inlineSearchService.query(queryDto);
    }
  }

  private isInputFocused(): boolean {
    const focusedElement = document.activeElement;
    const result = focusedElement && (focusedElement.tagName === 'INPUT' || focusedElement.tagName === 'TEXTAREA');
    return result ? result : false;
  }

  onUndoClick() {

  }

}
