import { Injectable } from "@angular/core";
import { BehaviorSubject } from "rxjs";
import { FileDTOReceived } from "./dtos/file-dto-received";
import { invoke } from "@tauri-apps/api/core";

@Injectable({ 'providedIn': 'root' })
export class DirectoryNavigatorService {
    private currentDirSubject = new BehaviorSubject<string>('C:\\');
    public currentDir$ = this.currentDirSubject.asObservable();

    private currentFilesSubject = new BehaviorSubject<FileDTOReceived[]>([]);
    public currentFiles$ = this.currentFilesSubject.asObservable();

    constructor() { }

    setCurrentDir(dir: string) {
        this.currentDirSubject.next(this.formatPathIntoDir(dir));
        this.setDriveFiles();
    }

    setDriveFiles() {
        invoke<FileDTOReceived[]>("get_files_as_dtos", { directory: this.currentDirSubject.getValue() }).then((files) => {
            this.currentFilesSubject.next(files);
        });
    }

    formatPathIntoDir(path: string): string {
        invoke<string | undefined>("format_path_into_dir", { path: path }).then((newPath) => {
            if (newPath != undefined)
                return newPath;
        });
        return path;
    }
}