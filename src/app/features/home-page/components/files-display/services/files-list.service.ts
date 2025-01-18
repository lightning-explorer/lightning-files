import { Injectable } from "@angular/core";
import { defaultFileState, FileState } from "../../file-result/file-state";
import { BehaviorSubject, Observable } from "rxjs";
import { FileModel } from "@core/models/file-model";

@Injectable()
export class FilesListService {
  /** Associates the file path with the actual file */
  private fileMaps = new Map<string, BehaviorSubject<FileState>>();
  private filesSubject = new BehaviorSubject<FileState[]>([]);

  constructor() {}

  /** Get current state or create a new reactive state for a file */
  private getOrCreateFileStateSubject(
    file: FileModel
  ): BehaviorSubject<FileState> {
    const path = file.FilePath;
    if (!this.fileMaps.has(path)) {
      const initialState = defaultFileState(file);
      this.fileMaps.set(path, new BehaviorSubject(initialState));
    }
    return this.fileMaps.get(path)!;
  }

  /** Update or add a file state reactively */
  updateFileState(file: FileModel, state: Partial<FileState>) {
    const subject = this.getOrCreateFileStateSubject(file);
    const currentState = subject.value;
    subject.next({ ...currentState, ...state }); 
  }

  /** Removes all current files and replaces them with new ones */
  setFiles(files: FileState[]) {
    // Complete old state subjects
    this.fileMaps.forEach((subject) => subject.complete());
    this.fileMaps.clear();

    // Notify observers of the new list of files
    this.filesSubject.next(files);
  }

  /** Stores the files with default state */
  setFilesDefault(files: FileModel[]){
    const state = files.map(x=>defaultFileState(x));
    this.setFiles(state);
  }

  /** Observe the state of a specific file */
  observeFileState(file:FileModel): Observable<FileState> {
    return this.getOrCreateFileStateSubject(file).asObservable();
  }

  /** Observe the entire list of files */
  observeAllFiles(): Observable<FileState[]> {
    return this.filesSubject.asObservable();
  }
}
