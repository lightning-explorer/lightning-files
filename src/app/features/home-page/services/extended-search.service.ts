import { Injectable, OnDestroy, OnInit } from "@angular/core";
import { SearchParamsDTO } from "@core/dtos/output/search-params-dto";
import { StreamingSearchParamsDTO } from "@core/dtos/output/streaming-search-params-dtos";
import { FileModel } from "@core/models/file-model";
import { LocalStreamingSearchService } from "@core/services/search/text/local-streaming-search.service";
import { BehaviorSubject, Subscription } from "rxjs";

@Injectable()
export class ExtendedSearchService implements OnDestroy {
    subscription = new Subscription;

    private filesSubject = new BehaviorSubject<FileModel[]>([]);
    files$ = this.filesSubject.asObservable();

    constructor(private searchService: LocalStreamingSearchService) {
        this.searchService.files$.subscribe(files => {
            this.filesSubject.next(files);
        });
    }

    ngOnDestroy(): void {
        this.subscription.unsubscribe();
    }

    async search(value: string) {
        let searchParams: SearchParamsDTO = {
            FilePath: value,
            NumResults: 200
        }

        let streamParams: StreamingSearchParamsDTO = {
            StreamIdentifier: "search",
            StartingSize: 10,
            NumEvents: 10,
            Params: searchParams,
        }
        this.searchService.query(streamParams);
    }
}