import {
  AfterViewInit,
  ChangeDetectorRef,
  Component,
  ElementRef,
  NgZone,
  OnDestroy,
  ViewChild,
} from "@angular/core";
import { MatIconModule } from "@angular/material/icon";
import { DirectoryHistoryService } from "src/app/features/home-page/services/directory-history.service";
import { DirectoryNavigatorService } from "../../services/directory-navigator.service";
import { PinnedFilesHeaderComponent } from "./pinned-files-header/pinned-files-header.component";
import { TabsHolderComponent } from "./tabs-holder/tabs-holder.component";
import { DropdownButtonModalComponent } from "../../../../shared/components/buttons/dropdown-button-modal/dropdown-button-modal.component";
import { IconifyIconModule } from "../../../../shared/components/icons/IconifyIcons/icon.module";
import { CommonModule } from "@angular/common";
import {
  UtilButtonType,
  UtilButtonComponent,
} from "./util-button/util-button.component";
import { CurrentDirectoryBarComponent } from "./current-directory-bar/current-directory-bar.component";
import { MutedButtonComponent } from "../../../../shared/components/buttons/muted-button/muted-button.component";
import { SearchbarComponent } from "../searchbar/searchbar.component";

@Component({
  selector: "app-top-header",
  standalone: true,
  imports: [
    CommonModule,
    MatIconModule,
    PinnedFilesHeaderComponent,
    TabsHolderComponent,
    IconifyIconModule,
    UtilButtonComponent,
    CurrentDirectoryBarComponent,
    SearchbarComponent
],
  templateUrl: "./top-header.component.html",
  styleUrl: "./top-header.component.css",
})
export class TopHeaderComponent implements AfterViewInit, OnDestroy {
  @ViewChild("toolbarButtonContainer") toolbarButtonContainer!: ElementRef;
  utilButtons: UtilButtonType[] = [
    "new",
    "copy",
    "paste",
    "cut",
    "rename",
    "trash",
  ];

  visibleUtilButtons: UtilButtonType[] = [];
  overflowingUtilButtons: UtilButtonType[] = [];

  private resizeObserver!: ResizeObserver;

  constructor(
    private directoryService: DirectoryNavigatorService,
    private directoryHistoryService: DirectoryHistoryService,
    private ngZone: NgZone,
    private cdr: ChangeDetectorRef
  ) {}

  ngAfterViewInit(): void {
    this.resizeObserver = new ResizeObserver(() => {
      this.ngZone.run(() => {
        this.updateVisibleItems();
      });
    });

    this.resizeObserver.observe(this.toolbarButtonContainer.nativeElement);

    this.updateVisibleItems();
  }

  ngOnDestroy(): void {
    if (this.resizeObserver) this.resizeObserver.disconnect();
    this.cdr.detach();
  }

  updateVisibleItems() {
    this.visibleUtilButtons.length = 0;
    this.overflowingUtilButtons.length = 0;
    this.utilButtons.forEach((x) => {
      this.visibleUtilButtons.push(x);
    });
    this.cdr.detectChanges();

    const container = this.toolbarButtonContainer.nativeElement;
    const containerWidth = container.offsetWidth;

    const toolbarButtons = Array.from(
      container.querySelectorAll(".toolbar-button")
    ) as HTMLElement[];
    const padding = 1.2; // Arbitrary padding
    const elementWidths = toolbarButtons.map(
      (element) => element.offsetWidth * padding
    );
    let currentWidth = elementWidths.reduce((sum, width) => sum + width, 0);

    while(currentWidth > containerWidth){
      currentWidth -= elementWidths.pop()!;
      const removedElement = this.visibleUtilButtons.pop()!;
      this.overflowingUtilButtons.push(removedElement);
    }
  }

  async onNavigateBackDirectoryClick() {
    let parent = await this.directoryService.getParentDirectory();
    await this.directoryService.setCurrentDir(parent);
  }

  async onUndoClick() {
    this.directoryHistoryService.undo();
  }

  async onRedoClick() {
    this.directoryHistoryService.redo();
  }
}

// A:6 * B:5 * C:3 = D

