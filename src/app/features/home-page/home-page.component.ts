import { Component, HostListener, OnInit } from '@angular/core';

import { CommonModule } from '@angular/common';

import { SidebarComponent } from "./components/sidebar/sidebar.component";
import { FilesDisplayComponent } from "./components/files-display/files-display.component";
import { MatIconModule } from '@angular/material/icon';

import { TopHeaderComponent } from "./components/top-header/top-header.component";
import { PinnedFilesHeaderComponent } from "./components/pinned-files-header/pinned-files-header.component";
import { HomePageService, SubPage } from './home-page.service';
import { ExtendedSearchComponent } from "./pages/extended-search/extended-search.component";
import { ExtendedSearchService } from './pages/extended-search/extended-search.service';

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
  pages = {
    main:FilesDisplayComponent,
    extendedSearch: ExtendedSearchComponent
  }
  get currentPage(){
    return this.pages[this.page] || null;
  }

  constructor(private homePageService: HomePageService, private extendedSearchService: ExtendedSearchService) {

    this.homePageService.page$.subscribe(page =>
      this.page = page
    );
  }

  ngOnInit(): void {

  }

}
