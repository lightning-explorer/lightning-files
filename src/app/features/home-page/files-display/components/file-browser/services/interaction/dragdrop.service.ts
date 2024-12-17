import { ElementRef, Injectable } from '@angular/core';
import { FileModel } from '@core/models/file-model';

@Injectable()
export class DragDropService {
    draggingItemsTo = ""; // this is a directory path
    draggedItems: Set<FileModel> = new Set();

    constructor() { }

    onDragStart(event: DragEvent, items: Set<FileModel>, dragPreview: ElementRef) {
        this.draggedItems = items;

        const previewElement = dragPreview.nativeElement;
        event.dataTransfer?.setDragImage(previewElement, 0, 0);

        event.dataTransfer?.setData('text/plain', JSON.stringify(items));
        event.dataTransfer!.effectAllowed = 'move';
    }

    onDragOver(event: DragEvent) {
        event.preventDefault();
        event.dataTransfer!.dropEffect = 'move';
    }

    onDrop(event: DragEvent, targetItem: FileModel, maxNumDroppable: number): boolean {
        // You can't drag a folder into itself
        if (this.draggedItems.has(targetItem))
            return true;

        this.draggingItemsTo = targetItem.FilePath;
        event.preventDefault();
        if (targetItem.IsDirectory) {
            if (this.draggedItems.size > maxNumDroppable) {
                return false;
            }
            this.moveItems(targetItem);
        }
        return true;
    }

    moveItems(targetDirectory: FileModel) {
        this.draggedItems.forEach(item => {
            const file = item.FilePath;
            const move_to = targetDirectory.FilePath;
            if (file != move_to) {
                console.log(`Moved ${item.FilePath} to ${targetDirectory.FilePath}`);
            }
        });
    }
}
