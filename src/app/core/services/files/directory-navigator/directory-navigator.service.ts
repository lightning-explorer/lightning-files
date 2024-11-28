import { Injectable } from "@angular/core";
import { BehaviorSubject } from "rxjs";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { defaultParams, GetFilesParamsModel } from "./models/get-files-params";
import { FileModel } from "../../../models/file-model";

@Injectable({ 'providedIn': 'root' })
export class DirectoryNavigatorService {
    private currentDirSubject = new BehaviorSubject<string>('C:\\');
    public currentDir$ = this.currentDirSubject.asObservable();

    private currentFilesSubject = new BehaviorSubject<FileModel[]>([]);
    public currentFiles$ = this.currentFilesSubject.asObservable();

    constructor() { }

    async setCurrentDir(dir: string, params?: GetFilesParamsModel) {
        this.currentDirSubject.next(await this.formatPathIntoDir(dir, this.currentDirSubject.getValue()));
        await this.setDriveFiles(params);
    }

    async setDriveFiles(params?: GetFilesParamsModel) {
        if (!params)
            params = defaultParams();

        this.currentFilesSubject.next([]);

        const unlisten = await listen<FileModel>("sys_file_model", (event) => {
            const updatedFiles = [...this.currentFilesSubject.getValue(), event.payload];
            this.currentFilesSubject.next(updatedFiles);
        })

        try {
            await invoke("get_files_as_models", { directory: this.currentDirSubject.getValue(), params });
        }
        catch (err) {
            console.log("Error setting files", err)
        }
        finally {
            unlisten();
        }
    }

    async formatPathIntoDir(path: string, prevPath: string): Promise<string> {
        return await invoke<string | undefined>("format_path_into_dir", { path: path }).then((newPath) => {
            return newPath == undefined ? prevPath : newPath;
        });
    }

    async getDirectoryPath(): Promise<string> {
        return invoke<string>("get_directory_path", {
            filePath:
                this.currentDirSubject.getValue()
        }).then(path =>
            path
        )
    }

    async getParentDirectory(): Promise<string> {
        return invoke<string>("get_parent_directory", {
            filePath:
                this.currentDirSubject.getValue()
        }).then(path =>
            path
        )
    }

    async getRootDirectory(): Promise<string> {
        return invoke<string>("get_root_path", {
            filePath:
                this.currentDirSubject.getValue()
        }).then(path =>
            path
        )
    }

    async isPathAFile(filePath: string): Promise<boolean> {
        return invoke<boolean>("is_path_a_file", {
            filePath
        }).then(result =>
            result
        )
    }

    async openFileCmd(filePath: string): Promise<boolean> {
        return invoke<void>("open_file", {
            filePath
        }).then(() =>
            true
        ).catch(() =>
            false
        )
    }
}