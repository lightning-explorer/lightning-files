import { Injectable } from "@angular/core";
import { FileModel } from "../../../models/file-model";
import { BehaviorSubject } from "rxjs";
import { StreamingSearchParamsDTO } from "../../../dtos/output/streaming-search-params-dtos";
import { TauriCommandsService } from "../../tauri/commands.service";

@Injectable({ 'providedIn': 'root' })
export class LocalStreamingSearchService {
    private filesSubject = new BehaviorSubject<FileModel[]>([]);
    public files$ = this.filesSubject.asObservable();

    constructor(private commandsService: TauriCommandsService) { }

    async query(params: StreamingSearchParamsDTO) {
        this.filesSubject.next([]);
        await this.commandsService.searchIndexQueryStreaming(params,
            (files) => {
                const updatedFiles = [...this.filesSubject.getValue(), ...files];
                this.filesSubject.next(updatedFiles);
            }
        );
    }
}