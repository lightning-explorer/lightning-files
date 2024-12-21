import { Injectable } from "@angular/core";
import { FileState } from "../file-result/file-state";
import { BehaviorSubject, Observable } from "rxjs";
import { FileModel } from "@core/models/file-model";

@Injectable()
export class FilesListService {
  private fileStates = new Map<FileModel, FileState>();
  private filesSubject = new BehaviorSubject<FileModel[]>([]);

  constructor() {
    this.updateFileModelsSubject();
  }

  // Helper to update the subject with the current list of FileModels
  private updateFileModelsSubject() {
    this.filesSubject.next(Array.from(this.fileStates.keys()));
  }

  getFileState(file: FileModel): FileState {
    if (!this.fileStates.has(file)) {
      // Initialize default state if it doesn't exist
      this.fileStates.set(file, this.defaultFileState());
    }
    return this.fileStates.get(file)!;
  }

  updateFileState(file: FileModel, state: Partial<FileState>) {
    const currentState = this.getFileState(file);
    this.fileStates.set(file, { ...currentState, ...state });
    this.updateFileModelsSubject();
  }

  /** Removes all the current files and replaces them with new ones */
  setState(files:FileModel[]){
    
  }

  /** Keep track of the state for one specific file */
  observeFileState(file: FileModel): Observable<FileState> {
    return new BehaviorSubject(this.getFileState(file)).asObservable();
  }

  observeAllFiles(): Observable<FileModel[]> {
    return this.filesSubject.asObservable();
  }

  private defaultFileState(): FileState {
    return {
      highlightedText: "",
      draggedOver: false,
      hide: false,
    };
  }
}
