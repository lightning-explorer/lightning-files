
import { Injectable, OnDestroy, OnInit } from "@angular/core";
import { FileModel } from "@core/models/file-model";
import { PersistentConfigService } from "@core/services/persistence/config.service";
import { BehaviorSubject, Subscription } from "rxjs";


@Injectable()
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

    async pinFile(file: FileModel) {
        this.configService.update("pinnedFiles", [...this.pinnedFilesSubject.getValue(), file]);
        await this.configService.save();
    }

    async unpinFile(file: FileModel) {
        this.configService.update("pinnedFiles", this.pinnedFilesSubject.getValue().filter(x => x.FilePath != file.FilePath));
        await this.configService.save();
    }

    ngOnDestroy(): void {
        this.subscription.unsubscribe();
    }
}