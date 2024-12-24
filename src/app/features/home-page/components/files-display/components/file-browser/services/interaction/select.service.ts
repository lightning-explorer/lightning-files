import { Injectable } from "@angular/core";
import { FileModel } from "@core/models/file-model";
import { BehaviorSubject } from "rxjs";
import { DirectoryNavigatorService } from "src/app/features/home-page/services/directory-navigator.service";
import { FileOperationsService } from "src/app/features/home-page/services/file-operations.service";

/**
 * Handles both single click select and multiselect
 */
@Injectable()
export class SelectService {
  private selectedIndicesSubject = new BehaviorSubject<Set<number>>(new Set());
  private selectedItemsSubject = new BehaviorSubject<FileModel[]>([]);

  selectedIndices$ = this.selectedIndicesSubject.asObservable();
  selectedItems$ = this.selectedItemsSubject.asObservable();

  lastSelectedIndex: number | null = null;

  constructor(
    private directoryService: DirectoryNavigatorService,
    private fileOperationsService:FileOperationsService
  ) {}

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
    this.clearSelection();
    await this.fileOperationsService.openOrNavigateToFile(file);
  }

  selectRange(start: number, end: number) {
    // Ensure the range goes from the lower to the higher index
    const [from, to] = start < end ? [start, end] : [end, start];
    for (let i = from; i <= to; i++) {
      const s = this.selectedIndicesSubject.getValue();
      s.add(i);
      this.selectedIndicesSubject.next(s);
    }
  }

  toggleSelection(index: number) {
    const s = this.selectedIndicesSubject.getValue();
    if (s.has(index)) {
      s.delete(index);
    } else {
      s.add(index);
    }
    this.selectedIndicesSubject.next(s);
  }

  clearSelection() {
    this.selectedIndicesSubject.next(new Set());
  }

  selectIndex(index: number) {
    const s = this.selectedIndicesSubject.getValue();
    s.add(index);
    this.selectedIndicesSubject.next(s);
  }

  populateSelected(files: FileModel[]) {
    const s = this.selectedIndicesSubject.getValue();
    const sortedIndices = Array.from(s).sort((a, b) => a - b);
    let res: FileModel[] = [];
    sortedIndices.forEach((x) => {
      const item = files.at(x);
      if (item) res.push(item);
    });
    this.selectedItemsSubject.next(res);
  }
}
