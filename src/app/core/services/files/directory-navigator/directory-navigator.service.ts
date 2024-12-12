import { Injectable } from "@angular/core";
import { BehaviorSubject } from "rxjs";
import { GetFilesParamsModel } from "./models/get-files-params";
import { TauriCommandsService } from "../../tauri/commands.service";
import { FileModel } from "../../../models/file-model";
import { DirectoryMetadata, newDirMetadataDefault } from "./models/directory-metadata";

@Injectable({ 'providedIn': 'root' })
export class DirectoryNavigatorService {

    private currentDirMetadataSubject = new BehaviorSubject<DirectoryMetadata>(newDirMetadataDefault());
    public currentDirMetadata$ = this.currentDirMetadataSubject.asObservable();

    // True if the service is trying to load in the files asynchronously
    private isLoadingSubject = new BehaviorSubject<boolean>(false);
    public isLoading$ = this.isLoadingSubject.asObservable();

    private currentFilesSubject = new BehaviorSubject<FileModel[]>([]);
    public currentFiles$ = this.currentFilesSubject.asObservable();

    constructor(private commandsService: TauriCommandsService) { }

    async setCurrentDir(dir: string, params?: GetFilesParamsModel) {
        const currentMeta = this.currentDirMetadataSubject.getValue();
        this.currentDirMetadataSubject.next({
            ...currentMeta,
            isAccessible: await this.commandsService.isDirectoryAccessible(dir),
            directory: await this.commandsService.formatPathIntoDir(dir),
        });
        this.isLoadingSubject.next(true);
        await this.setDriveFiles(params);
        this.isLoadingSubject.next(false);
    }

    async setDriveFiles(params?: GetFilesParamsModel) {
        const directory = this.currentDirMetadataSubject.getValue().directory;

        this.currentFilesSubject.next([]);
        await this.commandsService.getFilesAsModels(directory, (file) => {
            const updatedFiles = [...this.currentFilesSubject.getValue(), file];
            this.currentFilesSubject.next(updatedFiles);
        }, params);
    }

    async getDirectoryPath(): Promise<string> {
        return await this.commandsService.getDirectoryPath(this.currentDirMetadataSubject.getValue().directory);
    }

    async getParentDirectory(): Promise<string> {
        return await this.commandsService.getParentDirectory(this.currentDirMetadataSubject.getValue().directory);
    }

    async getRootDirectory(): Promise<string> {
        return await this.commandsService.getRootPath(this.currentDirMetadataSubject.getValue().directory);
    }

    async openFileCmd(filePath: string): Promise<boolean> {
        return await this.commandsService.openFile(filePath);
    }
}