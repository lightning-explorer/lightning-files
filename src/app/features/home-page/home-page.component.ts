import { Component, HostListener, OnInit } from "@angular/core";

import { CommonModule } from "@angular/common";

import { SidebarComponent } from "./components/sidebar/sidebar.component";
import { FilesDisplayComponent } from "./components/files-display/files-display.component";
import { MatIconModule } from "@angular/material/icon";

import { TopHeaderComponent } from "./components/top-header/top-header.component";
import { HomePageService, SubPage } from "./services/home-page.service";
import { ExtendedSearchComponent } from "./pages/extended-search/extended-search.component";
import { DirectoryNavigatorService } from "./services/directory-navigator.service";
import { DirectoryHistoryService } from "./services/directory-history.service";
import { PinService } from "./services/pin.service";
import { FileOperationsService } from "./services/file-operations.service";
import { HomePageSearchService } from "./services/home-page-search.service";
import { PersistentConfigService } from "@core/services/persistence/config.service";
import { SettingsComponent } from "./pages/settings/settings.component";
import { DirectoryWatcherService } from "./services/directory-watcher.service";
import { TabsService } from "./services/tabs.service";

@Component({
  selector: "app-home-page",
  standalone: true,
  imports: [
    CommonModule,
    SidebarComponent,
    MatIconModule,
    TopHeaderComponent,
  ],
  templateUrl: "./home-page.component.html",
  styleUrl: "./home-page.component.scss",
  providers: [
    HomePageService,
    HomePageSearchService,
    DirectoryNavigatorService,
    DirectoryHistoryService,
    DirectoryWatcherService,
    PinService,
    FileOperationsService,
    TabsService,
  ],
})
export class HomePageComponent implements OnInit {
  page: SubPage = "main";
  pages = {
    main: FilesDisplayComponent,
    extendedSearch: ExtendedSearchComponent,
    settings: SettingsComponent,
  };
  get currentPage() {
    return this.pages[this.page] || null;
  }

  constructor(
    private directoryNavService: DirectoryNavigatorService,
    private homePageService: HomePageService,
    private configService: PersistentConfigService
  ) {
    this.homePageService.page$.subscribe((page) => (this.page = page));
  }

  async ngOnInit(): Promise<void> {
    const lastDirAt = await this.configService.readOrSet("lastDirectoryAt", "C:\\");
    console.log("Here is the last directory at:");
    console.log(lastDirAt);
    const dir = lastDirAt ?? "C:\\";
    this.directoryNavService.setCurrentDir(dir);
  }
}
