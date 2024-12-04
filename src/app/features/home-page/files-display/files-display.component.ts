import { Component, ElementRef, Input, OnChanges, OnInit, SimpleChanges, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FileResultComponent } from "../file-result/file-result.component";
import { CdkVirtualScrollViewport, ScrollingModule } from '@angular/cdk/scrolling';
import { trigger, state, style, animate, transition } from '@angular/animations';
import { FileModel } from '../../../core/models/file-model';
import { InlineSearchService } from '../../../core/services/search/text/inline-search.service';
import { SelectService } from './services/select.service';
import { DragDropService } from './services/dragdrop.service';
import { MoveItemsPopupComponent } from "./components/move-items-popup/move-items-popup.component";
import { DirectoryNavigatorService } from '../../../core/services/files/directory-navigator/directory-navigator.service';
import { FileContextMenuService } from './services/context-menu.service';
import { ContextMenuComponent } from "../../../shared/components/popups/context-menu/context-menu.component";
import { InlineSearchBarComponent } from "./components/inline-search-bar/inline-search-bar.component";

@Component({
  selector: 'app-files-display',
  standalone: true,
  imports: [CommonModule, FileResultComponent, ScrollingModule, MoveItemsPopupComponent, ContextMenuComponent, InlineSearchBarComponent],
  providers: [SelectService, DragDropService, FileContextMenuService],
  templateUrl: './files-display.component.html',
  styleUrl: './files-display.component.scss',
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
export class FilesDisplayComponent implements OnInit, OnChanges {
  @ViewChild(CdkVirtualScrollViewport) viewport!: CdkVirtualScrollViewport;
  @ViewChild('dragPreview') dragPreview!: ElementRef;
  @ViewChild('moveItemsPopup') moveItemsPopup!: MoveItemsPopupComponent;
  @ViewChild('contextMenu') contextMenu!: ContextMenuComponent;
  @Input() files: FileModel[] = [];

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
    this.inlineSearchService.firstOccurenceOfQueryIndex$.subscribe(x =>
      this.inlineSearchToFirstOccurence(x)
    );
    this.directoryService.currentDir$.subscribe(x => {
      this.selectService.clearSelection();
      this.currentDirectory = x
    });
  }

  ngOnChanges(changes: SimpleChanges) {
    if (changes['files']) {
      this.hideAndFadeIn();
    }
  }

  hideAndFadeIn() {
    this.animationState = 'hidden';

    setTimeout(() => {
      this.animationState = 'visible';
      this.viewport.checkViewportSize();
    }, 100); // Match this to the duration of the hide animation
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
    this.selectService.onFileClick(index, event);
  }

  onFileDoubleClick(file: FileModel) {
    this.inlineSearchService.clearQuery();
    this.selectService.onFileDoubleClick(file);
  }

  onFileRightClick(file: FileModel, event: MouseEvent) {
    this.contextMenuService.openMenu(this.contextMenu, event, file);
  }

  onDragStart(event: DragEvent, index: number, item: FileModel) {
    this.selectService.populateSelected(this.files);
    const selectedSet = new Set(this.selectedItems);
    if (!selectedSet.has(item)) {
      this.selectService.clearSelection();
      this.selectService.toggleSelection(index);
      this.selectService.populateSelected(this.files);
    }
    this.dragService.onDragStart(event, selectedSet, this.dragPreview);
  }

  onDragOver(event: DragEvent) {
    this.dragService.onDragOver(event);
  }

  onDrop(event: DragEvent, targetItem: FileModel) {
    if (!this.dragService.onDrop(event, targetItem, 0)) {
      this.moveItemsPopup.open(this.currentDirectory, this.dragService.draggingItemsTo, this.selectService.selectedIndices.size, () => {
        this.dragService.moveItems(targetItem);
      });
    }
  }


}
