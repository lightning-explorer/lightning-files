import { AfterViewInit, Component, OnDestroy, OnInit, ViewChild } from "@angular/core";
import { DriveService } from "@core/services/files/drive.service";
import { DriveModel } from "@core/models/drive-model";
import { CommonModule } from "@angular/common";
import { DriveResultComponent } from "../drive-result/drive-result.component";
import { ToolbarComponent } from "./toolbar/toolbar.component";
import { DropdownButtonComponent } from "@shared/components/buttons/dropdown-button/dropdown-button.component";
import { Subscription } from "rxjs";
import {
  QuickAccessFilesService,
  QuickAccessPath,
} from "@core/services/files/quick-access.service";
import { QuickAccessShortcutComponent } from "../quick-access-shortcut/quick-access-shortcut.component";
import { DirectoryNavigatorService } from "../../services/directory-navigator.service";
import { HomePageService } from "../../services/home-page.service";
import { ColorThemeService } from "@core/services/customization/color-theme.service";
import { AppIconNameComponent } from "../../../../layout/app-icon-name/app-icon-name.component";
import { ExtendBarVerticalComponent } from "../../../../shared/components/draggable/extend-bar-vertical/extend-bar-vertical.component";
import { ButtonWSvgComponent } from "../../../../shared/components/buttons/button-w-svg/button-w-svg.component";

@Component({
  selector: "app-sidebar",
  standalone: true,
  imports: [
    CommonModule,
    DriveResultComponent,
    DropdownButtonComponent,
    QuickAccessShortcutComponent,
    ExtendBarVerticalComponent,
    ButtonWSvgComponent
],
  templateUrl: "./sidebar.component.html",
  styleUrl: "./sidebar.component.scss",
})
export class SidebarComponent implements OnInit, AfterViewInit, OnDestroy {
  subscription = new Subscription();

  previewWidth = 0;
  drives$ = this.driveService.drives$;
  quickAccessPaths$ = this.quickAccessService.quickAccessPaths$;

  @ViewChild(ExtendBarVerticalComponent) extendBar!: ExtendBarVerticalComponent;

  constructor(
    private directoryNavService: DirectoryNavigatorService,
    private homePageService: HomePageService,
    private driveService: DriveService,
    private quickAccessService: QuickAccessFilesService
  ) {}

  ngOnInit(): void {
    this.driveService.refreshDrives();
  }

  ngAfterViewInit(): void {
    this.subscription.add(
      this.extendBar.contentWidth$.subscribe((x) => {
        this.previewWidth = x;
      })
    );
  }

  ngOnDestroy(): void {
    this.subscription.unsubscribe();
  }

  homeButtonClicked() {
    this.directoryNavService.setCurrentDir("Home");
    this.toMainPage();
  }

  driveClicked(drive: DriveModel) {
    this.directoryNavService.setCurrentDir(drive.Name);
    this.toMainPage();
  }

  quickAccessShortcutClicked(path: QuickAccessPath) {
    this.directoryNavService.setCurrentDir(path.path);
    this.toMainPage();
  }

  private toMainPage() {
    this.homePageService.setPage("main");
  }
}
