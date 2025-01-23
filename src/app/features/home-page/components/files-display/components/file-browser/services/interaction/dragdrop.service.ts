import {
  ElementRef,
  Injectable,
  Renderer2,
  RendererFactory2,
} from "@angular/core";
import { FileModel } from "@core/models/file-model";
import { BehaviorSubject } from "rxjs";
import { FilesListService } from "../../../../services/files-list.service";
import { startDrag } from "@crabnebula/tauri-plugin-drag";
import { TauriCommandsService } from "@core/services/tauri/commands.service";

@Injectable()
export class DragDropService {
  private draggingItemsToSubject = new BehaviorSubject<FileModel | undefined>(
    undefined
  );
  private draggedItemsSubject = new BehaviorSubject<Set<FileModel>>(new Set());

  draggingItemsTo$ = this.draggingItemsToSubject.asObservable();
  draggedItems$ = this.draggedItemsSubject.asObservable();
  get numberOfItemsBeingDragged() {
    return this.draggedItemsSubject.getValue().size;
  }
  /** Returns true if the files were attempted to be dropped in a directory */
  get draggingItemsToADirectory(): boolean {
    return this.draggingItemsToSubject.getValue()?.IsDirectory ?? false;
  }

  constructor(
    private commandsService: TauriCommandsService,
    private filesListService: FilesListService
  ) {}

  onDragStart(event: DragEvent, items: Set<FileModel>) {
    event.preventDefault();
    items.forEach((f) =>
      this.filesListService.updateFileState(f, { hide: true })
    );
    //this.draggedItemsSubject.next(items);

    startDrag({item:Array.from(items).map(x=>x.FilePath),
     icon:'assets/icons/appicon.svg'
    },(result)=>{
      console.log(result);
    });

    //event.dataTransfer?.setData("text/plain", JSON.stringify([...items]));
    //event.dataTransfer!.effectAllowed = "move";
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

  onDrop(event: DragEvent, targetItem: FileModel) {
    // You can't drag a folder into itself
    if (this.draggedItemsSubject.getValue().has(targetItem)) return;

    this.filesListService.updateFileState(targetItem, { draggedOver: false });

    this.draggingItemsToSubject.next(targetItem);
    event.preventDefault();

    this.unhideAllDraggingItems();
    return;
  }

  onDragEnd(event: DragEvent, targetItem: FileModel) {
    this.unhideAllDraggingItems();
  }

  /** Attempt to move the items that were being dragged into their previous target */
  async moveDraggedItemsAsync() {
    const target = this.draggingItemsToSubject.getValue();
    // Ensure that the target exists and is a directory
    if (!target || !target.IsDirectory) return;

    this.draggedItemsSubject.getValue().forEach(async (item) => {
      const file = item.FilePath;
      const moveTo = target.FilePath;
      await this.commandsService.movePathIntoDirectory(moveTo,file);
      if (file != moveTo) {
        console.log(`Moved ${item.FilePath} to ${moveTo}`);
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
