import { AfterViewChecked, Component, ElementRef, EventEmitter, Input, OnChanges, OnDestroy, OnInit, Output, SimpleChanges, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FileResultComponent } from '../../../file-result/file-result.component';
import { CdkVirtualScrollViewport, ScrollingModule } from '@angular/cdk/scrolling';
import { MoveItemsPopupComponent } from '../move-items-popup/move-items-popup.component';
import { TopHeaderComponent } from '../../../top-header/top-header.component';
import { InlineSearchBarComponent } from '../inline-search-bar/inline-search-bar.component';
import { ContextMenuComponent } from '../../../../../shared/components/popups/context-menu/context-menu.component';
import { FilePreviewComponent } from '../file-preview/file-preview.component';
import { FolderLoaderComponent } from '../../../../../shared/components/loaders/folder-loader/folder-loader.component';
import { FileContextMenuService } from './services/context-menu.service';
import { DragDropService } from './services/dragdrop.service';
import { SelectService } from './services/select.service';
import { animate, state, style, transition, trigger } from '@angular/animations';
import { FileModel } from '../../../../../core/models/file-model';
import { InlineSearchService } from '../../../../../core/services/search/text/inline-search.service';
import { DirectoryNavigatorService } from '../../../../../core/services/files/directory-navigator/directory-navigator.service';
import { debounceTime, Subject, Subscription, tap } from 'rxjs';


@Component({
  selector: 'app-file-browser',
  standalone: true,
  imports: [CommonModule, FileResultComponent, ScrollingModule, MoveItemsPopupComponent, ContextMenuComponent, InlineSearchBarComponent, FolderLoaderComponent],
  providers: [SelectService, DragDropService, FileContextMenuService],
  templateUrl: './file-browser.component.html',
  styleUrl: './file-browser.component.css',
  animations: [
    trigger('fadeAnimation', [
      state('hidden', style({ opacity: 0, display: 'none' })),
      state('visible', style({ opacity: 1, display: 'block' })),
      transition('hidden => visible', [
        style({ display: 'block' }),
        animate('100ms ease-in')
      ]),
    ])
  ]
})
export class FileBrowserComponent implements OnInit, OnChanges, OnDestroy {
  subscription = new Subscription();
  @ViewChild(CdkVirtualScrollViewport) viewport!: CdkVirtualScrollViewport;
  // Ensures that the virtual scroller renders correctly and is refreshes to compensate

  @ViewChild('dragPreview') dragPreview!: ElementRef;
  @ViewChild('moveItemsPopup') moveItemsPopup!: MoveItemsPopupComponent;
  @ViewChild('contextMenu') contextMenu!: ContextMenuComponent;

  @Input() files: FileModel[] = [];
  @Input() isLoading: boolean = false;
  @Output() fileClickedOn = new EventEmitter<FileModel>();

  currentDirectory: string = "";
  animationState = 'visible';

  get selectedIndices(): Set<number> {
    return this.selectService.selectedIndices;
  }
  get selectedItems() {
    return this.selectService.selectedItems;
  }

  constructor(private inlineSearchService: InlineSearchService,
    private dragService: DragDropService,
    private selectService: SelectService,
    private directoryService: DirectoryNavigatorService,
    private contextMenuService: FileContextMenuService,
  ) { }

  ngOnInit(): void {

    this.subscription.add(this.inlineSearchService.firstOccurenceOfQueryIndex$.subscribe(x =>
      this.inlineSearchToFirstOccurence(x)
    ));

    this.subscription.add(this.directoryService.currentDir$.subscribe(x => {
      this.selectService.clearSelection();
      this.currentDirectory = x
    }));

    this.hideAndFadeIn();
  }

  ngOnChanges(changes: SimpleChanges) {
    if (changes['files']) {
      this.hideAndFadeIn();
    }
  }

  ngOnDestroy(): void {
    this.subscription.unsubscribe();
  }

  hideAndFadeIn() {
    this.animationState = 'hidden';

    setTimeout(() => {
      this.animationState = 'visible';
      this.viewport?.checkViewportSize();
    }, 100); // Match this to the duration of the hide animation

    // Ensure that the CDK viewport renders correctly
    for (let i = 0; i < 5; i++) {
      setTimeout(() => this.viewport.checkViewportSize(), 200 * (i + 1));
    }
  }

  // scroll to the first occurence of a file/directory with offset
  inlineSearchToFirstOccurence(index: number) {
    if (this.viewport) {
      let indexOffset = index - 6;
      if (indexOffset < 1)
        indexOffset = 0;
      this.viewport.scrollToIndex(indexOffset, 'smooth');
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
    const selectedSet = new Set(this.selectedItems);
    if (!selectedSet.has(item)) {
      this.selectService.clearSelection();
      this.selectService.toggleSelection(index);
      this.selectService.populateSelected(this.files);
    }
    this.dragService.onDragStart(event, selectedSet, this.dragPreview);
  }

  onFileDragOver(event: DragEvent) {
    this.dragService.onDragOver(event);
  }

  onFileDrop(event: DragEvent, targetItem: FileModel) {
    if (!this.dragService.onDrop(event, targetItem, 0)) {
      this.moveItemsPopup.open(this.currentDirectory, this.dragService.draggingItemsTo, this.selectService.selectedIndices.size, () => {
        this.dragService.moveItems(targetItem);
      });
    }
  }


}