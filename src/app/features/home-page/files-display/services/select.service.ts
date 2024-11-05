import { Injectable } from '@angular/core';
import { FileModel } from '../../models/FileModel';
import { DirectoryNavigatorService } from '../../../../core/services/files/directory-navigator.service';

@Injectable()
/**
 * Handles both single click select and multiselect
 */
export class SelectService {
  selectedIndices = new Set<number>();

  lastSelectedIndex: number | null = null;

  constructor(private directoryService: DirectoryNavigatorService) { }

  onFileClick(index: number, event: MouseEvent) {
    if (event.shiftKey && this.lastSelectedIndex !== null) {
      // Handle Shift + Click for range selection
      this.selectRange(this.lastSelectedIndex, index);
    } else {
      // Regular click to select single item
      if (event.ctrlKey) {
        // Toggle selection if Ctrl is held
        this.toggleSelection(index);
      } else {
        // Clear other selections and select this item
        this.clearSelection();
        this.selectIndex(index);
      }
      // Update lastSelectedIndex for future shift-click actions
      this.lastSelectedIndex = index;
    }
  }

  async onFileDoubleClick(file: FileModel) {
    const path = file.Dto.FilePath;
    this.clearSelection();
    if (await this.directoryService.isPathAFile(path)) {
      await this.directoryService.openFileCmd(path);
    } else {
      await this.directoryService.setCurrentDir(path);
    }
  }

  selectRange(start: number, end: number) {
    // Ensure the range goes from the lower to the higher index
    const [from, to] = start < end ? [start, end] : [end, start];
    for (let i = from; i <= to; i++) {
      this.selectedIndices.add(i);
    }
  }

  toggleSelection(index: number) {
    if (this.selectedIndices.has(index)) {
      this.selectedIndices.delete(index);
    } else {
      this.selectedIndices.add(index);
    }
  }

  clearSelection() {
    this.selectedIndices.clear();
  }

  selectIndex(index: number) {
    this.selectedIndices.add(index);
  }
}
