
import { Injectable, OnDestroy, OnInit } from "@angular/core";
import { BehaviorSubject, Subscription } from "rxjs";
import { FileModel } from "../../models/file-model";
import { PersistentConfigService } from "../persistence/config.service";

@Injectable({ 'providedIn': 'root' })
export class PinService implements OnDestroy {

    private subscription = new Subscription();
    private pinnedFilesSubject = new BehaviorSubject<FileModel[]>([]);
    public pinnedFiles$ = this.pinnedFilesSubject.asObservable();

    constructor(private configService: PersistentConfigService) {
        this.subscription.add(this.configService.config$.subscribe(x => {
            this.pinnedFilesSubject.next(x.pinnedFiles)
        }));
    }

    isFilePinned(file: FileModel): boolean {
        return this.pinnedFilesSubject.getValue().some(x => x.FilePath == file.FilePath);
    }

    pinFile(file: FileModel) {
        this.configService.update("pinnedFiles", [...this.pinnedFilesSubject.getValue(), file]);
    }

    unpinFile(file: FileModel) {
        this.configService.update("pinnedFiles", this.pinnedFilesSubject.getValue().filter(x => x.FilePath != file.FilePath));
    }

    ngOnDestroy(): void {
        this.subscription.unsubscribe();
    }
}