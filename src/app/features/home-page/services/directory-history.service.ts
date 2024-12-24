import { Injectable, OnDestroy } from "@angular/core";
import { DirectoryNavigatorService } from "./directory-navigator.service";
import { DirectoryMetadata } from "../../../core/models/directory-metadata";
import { Subscription } from "rxjs";

/**
 * Provides functionality for the undo and redo buttons
 */
@Injectable()
export class DirectoryHistoryService implements OnDestroy {
  subscription = new Subscription();

  private selfUpdated = false;
  private currentDir: string | undefined;
  private undoStack: string[] = [];
  private redoStack: string[] = [];

  constructor(private directoryNavService: DirectoryNavigatorService) {
    this.subscription.add(
      this.directoryNavService.currentDir$.subscribe((dir) => {
        this.updateNavigate(dir);
      })
    );
  }

  updateNavigate(newDirectory: string) {
    const oldDir = this.currentDir;
    this.currentDir = newDirectory;
    if (this.selfUpdated) {
      this.selfUpdated = false;
      return;
    }
    if (oldDir) this.undoStack.push(oldDir);
    this.redoStack.length = 0; // Clear the redo stack since it is now invalidated
  }

  /** Navigate to the previously visited directory, if any */
  undo() {
    if (this.undoStack.length == 0) return; // Nowhere to undo
    const lastDirectory = this.undoStack.pop();
    if (this.currentDir) this.redoStack.push(this.currentDir);

    this.selfUpdated = true;
    this.directoryNavService.setCurrentDir(lastDirectory!);
  }

  /**  Navigate back to the visited directory, if any */
  redo() {
    if (this.redoStack.length == 0) return; // Nowhere to redo
    const nextDirectory = this.redoStack.pop();
    if (this.currentDir) this.undoStack.push(this.currentDir);

    this.selfUpdated = true;
    this.directoryNavService.setCurrentDir(nextDirectory!);
  }

  ngOnDestroy(): void {
    this.subscription.unsubscribe();
  }
}
