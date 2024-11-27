import { Component, HostListener, OnInit } from '@angular/core';

import { CommonModule } from '@angular/common';
import { FormControl } from '@angular/forms';
import { SearchParamsDTO } from '../../core/dtos/output/search-params-dto';
import { SidebarComponent } from "./sidebar/sidebar.component";
import { CurrentDirectoryBarComponent } from "./current-directory-bar/current-directory-bar.component";
import { FilesDisplayComponent } from "./files-display/files-display.component";
import { DirectoryNavigatorService } from '../../core/services/files/directory-navigator/directory-navigator.service';
import { MatIconModule } from '@angular/material/icon';
import { InlineQueryDTO } from '../../core/dtos/output/inline-query-dto';
import { InlineSearchService } from '../../core/services/search/text/inline-search.service';
import { FileModel } from '../../core/models/file-model';
import { TopHeaderComponent } from "./top-header/top-header.component";
import { PinnedFilesHeaderComponent } from "./pinned-files-header/pinned-files-header.component";
import { fileDTOToModel } from '../../core/models/converters/FileDTOToModel';

// TODO:
// put search bar in Shared and then make a simpler one in features to manage its own state

@Component({
  selector: 'app-home-page',
  standalone: true,
  imports: [CommonModule, SidebarComponent, FilesDisplayComponent, MatIconModule, TopHeaderComponent, PinnedFilesHeaderComponent],
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

  @HostListener('window:keydown', ['$event'])
  async handleKeydown(event: KeyboardEvent) {
    this.inlineSearchService.handleKeydown(event, this.driveFiles);
  }

  onUndoClick() {

  }

}
