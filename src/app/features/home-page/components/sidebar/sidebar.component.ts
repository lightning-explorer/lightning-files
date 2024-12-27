import { Component, OnDestroy } from "@angular/core";
import { DriveService } from "@core/services/files/drive.service";
import { DriveModel } from "@core/models/drive-model";
import { CommonModule } from "@angular/common";
import { DriveResultComponent } from "../drive-result/drive-result.component";
import { ToolbarComponent } from "./toolbar/toolbar.component";
import { DropdownButtonComponent } from "@shared/components/buttons/dropdown-button/dropdown-button.component";
import { Subscription } from "rxjs";
import { QuickAccessFilesService, QuickAccessPath } from "@core/services/files/quick-access.service";
import { QuickAccessShortcutComponent } from "../quick-access-shortcut/quick-access-shortcut.component";
import { DirectoryNavigatorService } from "../../services/directory-navigator.service";
import { HomePageService } from "../../services/home-page.service";
import { ColorThemeService } from "@core/services/customization/color-theme.service";
import { AppIconNameComponent } from "../../../../layout/app-icon-name/app-icon-name.component";

@Component({
  selector: "app-sidebar",
  standalone: true,
  imports: [
    CommonModule,
    DriveResultComponent,
    ToolbarComponent,
    DropdownButtonComponent,
    QuickAccessShortcutComponent,
    AppIconNameComponent
],
  templateUrl: "./sidebar.component.html",
  styleUrl: "./sidebar.component.scss",
})
export class SidebarComponent {
  drives$ = this.driveService.drives$;
  quickAccessPaths$ = this.quickAccessService.quickAccessPaths$;

  constructor(
    private directoryNavService:DirectoryNavigatorService,
    private homePageService: HomePageService,
    private driveService: DriveService,
    private quickAccessService: QuickAccessFilesService,

    private themeService:ColorThemeService
  ) {
    driveService.refreshDrives();
  }

  drivesButtonClicked(){}

  driveClicked(drive:DriveModel) {
    this.directoryNavService.setCurrentDir(drive.Name);
    this.homePageService.setPage("main");
  }

  quickAccessShortcutClicked(path:QuickAccessPath){
    this.directoryNavService.setCurrentDir(path.path);
  }

}
