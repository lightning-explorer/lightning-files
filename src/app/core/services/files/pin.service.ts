
import { Injectable, OnInit } from "@angular/core";
import { BehaviorSubject } from "rxjs";
import { DriveModel } from "../../models/drive-model";
import { invoke } from "@tauri-apps/api/core";
import { FileModel } from "../../../features/home-page/models/FileModel";
import { PersistentConfigService } from "../persistence/config.service";

@Injectable({ 'providedIn': 'root' })
export class PinService {

    private pinnedFilesSubject = new BehaviorSubject<FileModel[]>([]);
    public pinnedFiles$ = this.pinnedFilesSubject.asObservable();

    constructor(private configService: PersistentConfigService) {
        this.configService.config$.subscribe(x => {
            this.pinnedFilesSubject.next(x.pinnedFiles)
        })
    }

    isFilePinned(file: FileModel): boolean {
        return this.pinnedFilesSubject.getValue().some(x => x.Dto.FilePath == file.Dto.FilePath);
    }

    pinFile(file: FileModel) {
        this.configService.update("pinnedFiles", [...this.pinnedFilesSubject.getValue(), file]);
    }

    unpinFile(file: FileModel) {
        this.configService.update("pinnedFiles", this.pinnedFilesSubject.getValue().filter(x => x.Dto.FilePath != file.Dto.FilePath));
    }
}