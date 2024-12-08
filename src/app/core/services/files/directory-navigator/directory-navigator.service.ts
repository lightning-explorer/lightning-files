import { Injectable } from "@angular/core";
import { BehaviorSubject } from "rxjs";
import { GetFilesParamsModel } from "./models/get-files-params";
import { TauriCommandsService } from "../../tauri/commands.service";
import { FileModel } from "../../../models/file-model";

@Injectable({ 'providedIn': 'root' })
export class DirectoryNavigatorService {
    private currentDirSubject = new BehaviorSubject<string>('C:\\');
    public currentDir$ = this.currentDirSubject.asObservable();

    // True if the service is trying to load in the files asynchronously
    private isLoadingSubject = new BehaviorSubject<boolean>(false);
    public isLoading$ = this.isLoadingSubject.asObservable();

    private currentFilesSubject = new BehaviorSubject<FileModel[]>([]);
    public currentFiles$ = this.currentFilesSubject.asObservable();

    constructor(private commandsService: TauriCommandsService) { }

    async setCurrentDir(dir: string, params?: GetFilesParamsModel) {
        this.currentDirSubject.next(await this.formatPathIntoDir(dir, this.currentDirSubject.getValue()));
        this.isLoadingSubject.next(true);
        await this.setDriveFiles(params);
        this.isLoadingSubject.next(false);
    }

    async setDriveFiles(params?: GetFilesParamsModel) {
        const directory = this.currentDirSubject.getValue();

        this.currentFilesSubject.next([]);
        await this.commandsService.getFilesAsModels(directory, (file) => {
            const updatedFiles = [...this.currentFilesSubject.getValue(), file];
            this.currentFilesSubject.next(updatedFiles);
        }, params);
    }

    async formatPathIntoDir(path: string, prevPath: string): Promise<string> {
        return await this.commandsService.formatPathIntoDir(path, prevPath);
    }

    async getDirectoryPath(): Promise<string> {
        return await this.commandsService.getDirectoryPath(this.currentDirSubject.getValue());
    }

    async getParentDirectory(): Promise<string> {
        return await this.commandsService.getParentDirectory(this.currentDirSubject.getValue());
    }

    async getRootDirectory(): Promise<string> {
        return await this.commandsService.getRootPath(this.currentDirSubject.getValue());
    }

    async openFileCmd(filePath: string): Promise<boolean> {
        return await this.commandsService.openFile(filePath);
    }
}