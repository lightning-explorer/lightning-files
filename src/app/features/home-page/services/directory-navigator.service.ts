import { Injectable } from "@angular/core";
import { BehaviorSubject } from "rxjs";
import { getFilesParams_DefaultParams, GetFilesParamsDTO } from "@core/dtos/get-files-params-dto";
import { TauriCommandsService } from "@core/services/tauri/commands.service";
import { FileModel } from "@core/models/file-model";
import { DirectoryMetadata, newDirMetadataDefault } from "@core/models/directory-metadata";
import { DirectoryHistoryService } from "./directory-history.service";

@Injectable()
export class DirectoryNavigatorService {

    private currentDirSubject = new BehaviorSubject<string>("");
    public currentDir$ = this.currentDirSubject.asObservable();

    private currentDirMetadataSubject = new BehaviorSubject<DirectoryMetadata>(newDirMetadataDefault());
    public currentDirMetadata$ = this.currentDirMetadataSubject.asObservable();

    // True if the service is trying to load in the files asynchronously
    private isLoadingSubject = new BehaviorSubject<boolean>(false);
    public isLoading$ = this.isLoadingSubject.asObservable();

    private currentFilesSubject = new BehaviorSubject<FileModel[]>([]);
    public currentFiles$ = this.currentFilesSubject.asObservable();

    constructor(private commandsService: TauriCommandsService) { }

    async setCurrentDir(dir: string, params?: GetFilesParamsDTO) {
        // avoid redundant emissions
        if (this.currentDirSubject.getValue() !== dir) {
            const currentMeta = this.currentDirMetadataSubject.getValue();
            this.currentDirMetadataSubject.next({
                ...currentMeta,
                isAccessible: await this.commandsService.isDirectoryAccessible(dir),
            });
            this.currentDirSubject.next(await this.commandsService.formatPathIntoDir(dir));
            this.isLoadingSubject.next(true);

            await this.setDriveFiles(params);

            this.isLoadingSubject.next(false);
        }
    }

    async setDriveFiles(params?: GetFilesParamsDTO) {
        const directory = this.currentDirSubject.getValue();

        if(!params) 
            params = getFilesParams_DefaultParams(); // No sorting logic or anything fancy

        this.currentFilesSubject.next([]);
        await this.commandsService.getFilesAsModels(directory, (file) => {
            const updatedFiles = [...this.currentFilesSubject.getValue(), file];
            this.currentFilesSubject.next(updatedFiles);
        }, params);
    }

    async isPathAFile(filePath:string):Promise<boolean>{
        return await this.commandsService.isPathAFile(filePath);
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

    getCurrentMetadata(): DirectoryMetadata {
        return this.currentDirMetadataSubject.getValue();
    }
}