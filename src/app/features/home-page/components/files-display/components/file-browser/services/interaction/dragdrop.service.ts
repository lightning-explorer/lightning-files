import {
  ElementRef,
  Injectable,
  Renderer2,
  RendererFactory2,
} from "@angular/core";
import { FileModel } from "@core/models/file-model";
import { FileResultComponent } from "../../../../../file-result/file-result.component";
import { BehaviorSubject } from "rxjs";
import { FilesListService } from "../../../../files-list.service";

@Injectable()
export class DragDropService {
  private draggingItemsToSubject = new BehaviorSubject<string>(""); // this is a directory path
  private draggedItemsSubject = new BehaviorSubject<Set<FileModel>>(new Set());

  draggingItemsTo$ = this.draggingItemsToSubject.asObservable();
  draggedItems$ = this.draggedItemsSubject.asObservable();

  constructor(private filesListService: FilesListService) {}

  onDragStart(
    event: DragEvent,
    items: Set<FileModel>,
    dragPreview: ElementRef
  ) {
    items.forEach((f) =>
      this.filesListService.updateFileState(f, { hide: true })
    );
    this.draggedItemsSubject.next(items);

    const previewElement = dragPreview.nativeElement;
    event.dataTransfer?.setDragImage(previewElement, 0, 0);

    event.dataTransfer?.setData("text/plain", JSON.stringify(items));
    event.dataTransfer!.effectAllowed = "move";
  }

  onDragOver(event: DragEvent, targetItem: FileModel) {
    if (targetItem.IsDirectory) {
      event.preventDefault();
      event.dataTransfer!.dropEffect = "move";

      this.filesListService.updateFileState(targetItem, { draggedOver: true });
    }
  }

  onDragLeave(event: DragEvent, targetItem: FileModel) {
    event.preventDefault();
    this.filesListService.updateFileState(targetItem, { draggedOver: false });
  }

  /** Returns `false` if the user tries to drop too many items in (a warning will be triggered) */
  onDrop(
    event: DragEvent,
    targetItem: FileModel,
    maxNumDroppable: number
  ): boolean {
    // You can't drag a folder into itself
    if (this.draggedItemsSubject.getValue().has(targetItem)) return true;

    this.filesListService.updateFileState(targetItem, { draggedOver: false });

    this.draggingItemsToSubject.next(targetItem.FilePath);
    event.preventDefault();

    if (targetItem.IsDirectory) {
      if (this.draggedItemsSubject.getValue().size > maxNumDroppable) {
        return false;
      }
      this.unhideAllDraggingItems();
      this.moveItems(targetItem);
    } else {
      this.unhideAllDraggingItems();
    }
    return true;
  }

  onDragEnd(event: DragEvent, targetItem: FileModel) {
    this.unhideAllDraggingItems();
  }

  moveItems(targetDirectory: FileModel) {
    this.draggedItemsSubject.getValue().forEach((item) => {
      const file = item.FilePath;
      const moveTo = targetDirectory.FilePath;
      if (file != moveTo) {
        console.log(`Moved ${item.FilePath} to ${targetDirectory.FilePath}`);
      }
    });
  }

  unhideAllDraggingItems() {
    const currentItems = this.draggedItemsSubject.getValue();
    currentItems.forEach((f) =>
      this.filesListService.updateFileState(f, { hide: false })
    );
    this.draggedItemsSubject.next(new Set([...currentItems]));
  }
}
