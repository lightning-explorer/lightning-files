import { Component, HostListener, OnInit } from '@angular/core';

import { CommonModule } from '@angular/common';

import { SidebarComponent } from "./sidebar/sidebar.component";
import { FilesDisplayComponent } from "./files-display/files-display.component";
import { MatIconModule } from '@angular/material/icon';

import { TopHeaderComponent } from "./top-header/top-header.component";
import { PinnedFilesHeaderComponent } from "./pinned-files-header/pinned-files-header.component";
import { HomePageService, SubPage } from './services/home-page.service';
import { ExtendedSearchComponent } from "./extended-search/extended-search.component";
import { ExtendedSearchService } from './services/extended-search.service';

// Home page has multiple subpages:
// * File browser
// * Extended search

// TODO:
// put search bar in Shared and then make a simpler one in features to manage its own state

@Component({
  selector: 'app-home-page',
  standalone: true,
  imports: [CommonModule, SidebarComponent, FilesDisplayComponent, MatIconModule, TopHeaderComponent, PinnedFilesHeaderComponent, ExtendedSearchComponent],
  templateUrl: './home-page.component.html',
  styleUrl: './home-page.component.scss',
  providers: [HomePageService, ExtendedSearchService]
})
export class HomePageComponent implements OnInit {
  page: SubPage = "main";

  constructor(private homePageService: HomePageService, private extendedSearchService: ExtendedSearchService) {

    this.homePageService.page$.subscribe(page =>
      this.page = page
    );
  }

  ngOnInit(): void {

  }

}
