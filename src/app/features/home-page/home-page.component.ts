import { Component, OnInit } from '@angular/core';
import { SearchEngineService } from '../../core/services/search-engine.service';
import { SearchbarComponent } from "./searchbar/searchbar.component";
import { FileDTOReceived } from '../../core/dtos/file-dto-received';
import { FileResultComponent } from "./file-result/file-result.component";
import { CommonModule } from '@angular/common';
import { FormControl } from '@angular/forms';
import { debounceTime } from 'rxjs';
import { SearchParamsDTO } from '../../core/dtos/search-params-dto';
import { SidebarComponent } from "./sidebar/sidebar.component";
import { CurrentDirectoryBarComponent } from "./current-directory-bar/current-directory-bar.component";
import { TestHtmlComponent } from "../../shared/test-html/test-html.component";
import { FilesDisplayComponent } from "./files-display/files-display.component";
import { invoke } from '@tauri-apps/api/core';
import { DirectoryNavigatorService } from '../../core/services/directory-navigator.service';
import { MatIconModule } from '@angular/material/icon';

@Component({
  selector: 'app-home-page',
  standalone: true,
  imports: [SearchbarComponent, FileResultComponent, CommonModule, SidebarComponent, CurrentDirectoryBarComponent, TestHtmlComponent, FilesDisplayComponent, MatIconModule],
  templateUrl: './home-page.component.html',
  styleUrl: './home-page.component.scss'
})
export class HomePageComponent implements OnInit {

  inputControl = new FormControl();

  searchResults: FileDTOReceived[] = [];
  driveFiles: FileDTOReceived[] = [];

  constructor(private searchEngineService: SearchEngineService,
    private directoryService: DirectoryNavigatorService
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

  onUndoClick() {

  }

}
