import { Injectable } from '@angular/core';
import { FileModel } from '../../../../../../core/models/file-model';
import { DirectoryNavigatorService } from '../../../../../../core/services/files/directory-navigator/directory-navigator.service';
import { FileCrawlerService } from '../../../../../../core/services/files/file_crawler.service';
import { isPathAFile } from '../../../../../../core/util/file/general';

@Injectable()
/**
 * Handles both single click select and multiselect
 */
export class SelectService {
  selectedIndices = new Set<number>();
  selectedItems: FileModel[] = [];

  lastSelectedIndex: number | null = null;

  constructor(private directoryService: DirectoryNavigatorService, private fileCrawlerService: FileCrawlerService) { }

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
    const path = file.FilePath;
    this.clearSelection();
    if (await isPathAFile(path)) {
      await this.directoryService.openFileCmd(path);
    } else {

      await this.directoryService.setCurrentDir(path);
      // When the user clicks on a directory, go ahead and add that directory to the crawler queue.
      // If the directory was indexed recently, then it will automatically get ignored
      await this.fileCrawlerService.addDirectoriesToQueue([{ DirPath: path, Priority: 0 }]);
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

  populateSelected(files: FileModel[]) {
    const sortedIndices = Array.from(this.selectedIndices).sort((a, b) => a - b);
    let res: FileModel[] = [];
    sortedIndices.forEach(x => {
      const item = files.at(x)
      if (item)
        res.push(item);
    });
    this.selectedItems = res;
  }
}
