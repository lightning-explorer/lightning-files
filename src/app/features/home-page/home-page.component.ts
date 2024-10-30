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
import { FileModel } from './models/FileModel';
import { fileDTOToModel } from './models/converters/FileDTOToModel';

// TODO:
// put search bar in Shared and then make a simpler one in features to manage its own state

@Component({
  selector: 'app-home-page',
  standalone: true,
  imports: [SearchbarComponent, FileResultComponent, CommonModule, SidebarComponent, CurrentDirectoryBarComponent, FilesDisplayComponent, MatIconModule],
  templateUrl: './home-page.component.html',
  styleUrl: './home-page.component.scss',
  providers: []
})
export class HomePageComponent implements OnInit {

  inputControl = new FormControl();
  driveFiles: FileModel[] = [];

  constructor(
    private directoryService: DirectoryNavigatorService,
    private inlineSearchService: InlineSearchService
  ) { }

  ngOnInit(): void {
    this.directoryService.setDriveFiles();

    this.directoryService.currentFiles$.subscribe(x =>
      this.driveFiles = x.map(x => fileDTOToModel(x))
    );
  }

  async onNavigateBackDirectoryClick() {
    let parent = await this.directoryService.getParentDirectory()
    await this.directoryService.setCurrentDir(parent);
  }


  @HostListener('window:keydown', ['$event'])
  async handleKeydown(event: KeyboardEvent) {
    this.inlineSearchService.handleKeydown(event, this.driveFiles);
  }

  onUndoClick() {

  }

}
