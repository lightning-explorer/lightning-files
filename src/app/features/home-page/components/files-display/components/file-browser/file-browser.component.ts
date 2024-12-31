import {
  Component,
  ElementRef,
  EventEmitter,
  HostListener,
  Input,
  OnChanges,
  OnDestroy,
  OnInit,
  Output,
  SimpleChanges,
  ViewChild,
} from "@angular/core";
import { CommonModule } from "@angular/common";
import { FileResultComponent } from "../../../file-result/file-result.component";
import {
  CdkVirtualScrollViewport,
  ScrollingModule,
} from "@angular/cdk/scrolling";
import { MoveItemsPopupComponent } from "./popups/move-items-popup/move-items-popup.component";
import { InlineSearchBarComponent } from "./inline-search-bar/inline-search-bar.component";
import { ContextMenuComponent } from "@shared/components/popups/context-menu/context-menu.component";
import { FolderLoaderComponent } from "@shared/components/loaders/folder-loader/folder-loader.component";
import { FileContextMenuService } from "./services/interaction/context-menu.service";
import { DragDropService } from "./services/interaction/dragdrop.service";
import { SelectService } from "./services/interaction/select.service";
import {
  animate,
  state,
  style,
  transition,
  trigger,
} from "@angular/animations";
import { FileModel } from "@core/models/file-model";
import { debounceTime, Subject, Subscription, tap } from "rxjs";
import { InlineSearchService } from "./services/inline-search.service";
import { FailedToMoveItemsPopupComponent } from "./popups/generic-err-popup/generic-err-popup.component";
import { FilesListService } from "../../services/files-list.service";
import { FileState } from "../../../file-result/file-state";
@Component({
  selector: "app-file-browser",
  standalone: true,
  imports: [
    CommonModule,
    FileResultComponent,
    ScrollingModule,
    MoveItemsPopupComponent,
    ContextMenuComponent,
    InlineSearchBarComponent,
    FailedToMoveItemsPopupComponent,
  ],
  providers: [
    SelectService,
    DragDropService,
    InlineSearchService,
    FileContextMenuService,
  ],
  templateUrl: "./file-browser.component.html",
  styleUrl: "./file-browser.component.css",
  animations: [
    trigger("fadeAnimation", [
      state("hidden", style({ opacity: 0, display: "none" })),
      state("visible", style({ opacity: 1, display: "block" })),
      transition("hidden => visible", [
        style({ display: "block" }),
        animate("100ms ease-in"),
      ]),
    ]),
  ],
})
export class FileBrowserComponent implements OnInit, OnDestroy {
  subscription = new Subscription();
  @ViewChild(CdkVirtualScrollViewport) viewport!: CdkVirtualScrollViewport;

  files: FileModel[] = [];
  states: FileState[] = [];

  @ViewChild("moveItemsPopup") moveItemsPopup!: MoveItemsPopupComponent;
  @ViewChild("contextMenu") contextMenu!: ContextMenuComponent;

  @Input() isLoading: boolean = false;
  @Output() fileClickedOn = new EventEmitter<FileModel>();

  animationState = "visible";

  selectedIndices: Set<number> = new Set();
  selectedItems: FileModel[] = [];

  constructor(
    private inlineSearchService: InlineSearchService, // Global service
    private filesListService: FilesListService,
    private dragService: DragDropService,
    private selectService: SelectService,
    private contextMenuService: FileContextMenuService
  ) {}

  ngOnInit(): void {
    this.subscription.add(
      this.filesListService.observeAllFiles().subscribe((x) => {
        this.hideAndFadeIn();
        this.files = x;
      })
    );

    this.subscription.add(
      this.filesListService
        .observeAllStates()
        .subscribe((x) => (this.states = x))
    );

    this.subscription.add(
      this.inlineSearchService.firstOccurenceOfQueryIndex$.subscribe((x) =>
        this.inlineSearchToFirstOccurence(x)
      )
    );

    this.subscription.add(
      this.selectService.selectedIndices$.subscribe(
        (x) => (this.selectedIndices = x)
      )
    );

    this.subscription.add(
      this.selectService.selectedItems$.subscribe(
        (x) => (this.selectedItems = x)
      )
    );

    this.hideAndFadeIn();
  }

  ngOnDestroy(): void {
    this.subscription.unsubscribe();
  }

  hideAndFadeIn() {
    this.animationState = "hidden";

    setTimeout(() => {
      this.animationState = "visible";
      this.viewport?.checkViewportSize();
    }, 100); // Match this to the duration of the hide animation

    // Ensure that the CDK viewport renders correctly
    // for (let i = 0; i < 3; i++) {
    //   setTimeout(() => {
    //     this.viewport.checkViewportSize();
    //   }, 200 * (i + 1));
    // }
  }

  // scroll to the first occurence of a file/directory with offset
  inlineSearchToFirstOccurence(index: number) {
    if (this.viewport) {
      let indexOffset = index - 6;
      if (indexOffset < 1) indexOffset = 0;
      this.viewport.scrollToIndex(indexOffset, "smooth");
    }
  }

  onFileClick(index: number, event: MouseEvent) {
    this.fileClickedOn.emit(this.files[index]);
    this.selectService.onFileClick(index, event);
  }

  onFileDoubleClick(file: FileModel) {
    this.inlineSearchService.clearQuery();
    this.selectService.onFileDoubleClick(file);
  }

  onFileRightClick(file: FileModel, event: MouseEvent) {
    this.contextMenuService.openMenu(this.contextMenu, event, file);
  }

  onFileDragStart(event: DragEvent, index: number, item: FileModel) {
    this.selectService.populateSelected(this.files);
    let selectedSet = new Set(this.selectedItems);
    if (!selectedSet.has(item)) {
      this.selectService.clearSelection();
      this.selectService.toggleSelection(index);
      this.selectService.populateSelected(this.files);
    }
    selectedSet = new Set(this.selectedItems);
    this.dragService.onDragStart(event, selectedSet);
  }

  onFileDragEnd(event: DragEvent, targetItem: FileModel) {
    this.dragService.onDragEnd(event, targetItem);
  }

  onFileDragOver(event: DragEvent, targetItem: FileModel) {
    this.dragService.onDragOver(event, targetItem);
  }

  onFileDragLeave(event: DragEvent, targetItem: FileModel) {
    this.dragService.onDragLeave(event, targetItem);
  }

  onFileDrop(event: DragEvent, targetItem: FileModel) {
    this.dragService.onDrop(event, targetItem);
    if (this.dragService.numberOfItemsBeingDragged > 0) {
      this.moveItemsPopup.isVisible = true;
      // TODO: possibly put this logic in a service
      this.moveItemsPopup.onYesClicked = () => {
        this.dragService.moveDraggedItems();
      };
      this.moveItemsPopup.onDestroy = () => {
        this.dragService.unhideAllDraggingItems();
      };
    }else{
      this.dragService.moveDraggedItems();
    }
  }

  @HostListener("window:keydown", ["$event"])
  async handleKeydown(event: KeyboardEvent) {
    this.inlineSearchService.handleKeydown(event, this.files);
  }

  // Prevent the stop sign from showing up if dragging a file over the background
  onMainDragOver(event: DragEvent) {
    event.preventDefault();
  }
}
