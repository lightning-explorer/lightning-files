import {
  AfterViewInit,
  Component,
  ElementRef,
  NgZone,
  OnDestroy,
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
  ],
  templateUrl: "./top-header.component.html",
  styleUrl: "./top-header.component.css",
})
export class TopHeaderComponent implements AfterViewInit, OnDestroy {
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
    private elementRef: ElementRef
  ) {}

  ngAfterViewInit(): void {
    const toolbar = this.elementRef.nativeElement as HTMLElement;

    this.resizeObserver = new ResizeObserver(() => {
      this.ngZone.run(() => {
        this.updateVisibleItems();
      });
    });

    this.resizeObserver.observe(toolbar);

    this.updateVisibleItems();
  }

  ngOnDestroy(): void {
    if (this.resizeObserver) this.resizeObserver.disconnect();
  }

  updateVisibleItems() {
    const toolbar = this.elementRef.nativeElement as HTMLElement;

    const availableWidth = toolbar.offsetWidth;

    const buttonWidth = 22;
    let usedWidth = 180;

    this.visibleUtilButtons = [];
    this.overflowingUtilButtons = [];

    for (const item of this.utilButtons) {
      if (usedWidth + buttonWidth <= availableWidth) {
        this.visibleUtilButtons.push(item);
        usedWidth += buttonWidth;
      } else {
        this.overflowingUtilButtons.push(item);
      }
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
