import { ElementRef, Injectable } from '@angular/core';
import { FileModel } from '@core/models/file-model';
import { FileResultComponent } from '../file-result.component';

@Injectable()
export class DragDropService {
    draggingItemsTo = ""; // this is a directory path
    draggedItems: Set<FileModel> = new Set();

    constructor() { }

    onDragStart(event: DragEvent, items: Set<FileModel>, dragPreview: ElementRef) {
        items.forEach(x=>x.Variables.ShouldHide=true);
        this.draggedItems = items;

        const previewElement = dragPreview.nativeElement;
        event.dataTransfer?.setDragImage(previewElement, 0, 0);

        event.dataTransfer?.setData('text/plain', JSON.stringify(items));
        event.dataTransfer!.effectAllowed = 'move';
    }

    onDragOver(event: DragEvent, targetItem: FileModel) {
        if(targetItem.IsDirectory){
            event.preventDefault();
            event.dataTransfer!.dropEffect = 'move';
            targetItem.Variables.DraggedOver = true;
        } 
    }

    onDragLeave(event: DragEvent, targetItem: FileModel) {
        event.preventDefault();
        targetItem.Variables.DraggedOver = false;
    }

    /** Returns `false` if the user tries to drop too many items in (a warning will be triggered) */
    onDrop(event: DragEvent, targetItem: FileModel, maxNumDroppable: number): boolean {
        // You can't drag a folder into itself
        if (this.draggedItems.has(targetItem))
            return true;

        targetItem.Variables.DraggedOver = false;

        this.draggingItemsTo = targetItem.FilePath;
        event.preventDefault();

        if (targetItem.IsDirectory) {
            if (this.draggedItems.size > maxNumDroppable) {
                return false;
            }
            this.unhideAllDraggingItems();
            this.moveItems(targetItem);
        }else{
            this.unhideAllDraggingItems();
        }
        return true;
    }

    moveItems(targetDirectory: FileModel) {
        this.draggedItems.forEach(item => {
            const file = item.FilePath;
            const moveTo = targetDirectory.FilePath;
            if (file != moveTo) {
                console.log(`Moved ${item.FilePath} to ${targetDirectory.FilePath}`);
            }
        });
    }

    unhideAllDraggingItems(){
        this.draggedItems.forEach(x=>x.Variables.ShouldHide=false);
    }
}
