import { HttpClient } from "@angular/common/http";
import { Injectable } from "@angular/core";
import { SearchParamsDTO } from "../../../dtos/output/search-params-dto";
import { environment } from "../../../../../environments/environment";
import { invoke } from "@tauri-apps/api/core";
import { FileModel } from "../../../models/file-model";
import { BehaviorSubject } from "rxjs";
import { listen } from "@tauri-apps/api/event";
import { StreamingSearchParamsDTO } from "../../../dtos/output/streaming-search-params-dtos";

@Injectable({ 'providedIn': 'root' })
export class LocalStreamingSearchService {
    private filesSubject = new BehaviorSubject<FileModel[]>([]);
    public files$ = this.filesSubject.asObservable();

    constructor() { }

    async query(params: StreamingSearchParamsDTO) {

        this.filesSubject.next([]);
        const eventName = `${params.StreamIdentifier}:search_result`

        const unlisten = await listen<FileModel[]>(eventName, (event) => {
            const updatedFiles = [...this.filesSubject.getValue(), ...event.payload];
            this.filesSubject.next(updatedFiles);
        });

        try {
            await invoke<Promise<void>>("search_index_query_streaming", { params });
        }
        catch (err) {
            console.log("Error performing streamed query", err)
        }
        finally {
            unlisten();
        }
    }
}