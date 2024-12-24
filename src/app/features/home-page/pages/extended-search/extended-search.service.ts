import { Injectable, OnDestroy, OnInit } from "@angular/core";
import { SearchParamsDTO } from "@core/dtos/search-params-dto";
import { StreamingSearchParamsDTO } from "@core/dtos/streaming-search-params-dtos";
import { FileModel } from "@core/models/file-model";
import { LocalStreamingSearchService } from "@core/services/search/text/local-streaming-search.service";
import { BehaviorSubject, last, Subscription } from "rxjs";

@Injectable()
export class ExtendedSearchService implements OnDestroy {
  subscription = new Subscription();

  private searchStringSubject = new BehaviorSubject<string>("");
  searchString$ = this.searchStringSubject.asObservable();

  private filesSubject = new BehaviorSubject<FileModel[]>([]);
  files$ = this.filesSubject.asObservable();

  constructor(private searchService: LocalStreamingSearchService) {
    this.searchService.files$.subscribe((files) => {
      this.filesSubject.next(files);
    });
  }

  ngOnDestroy(): void {
    this.subscription.unsubscribe();
  }

  async search(value: string) {
    const lastQuery = this.searchStringSubject.getValue();
    if (lastQuery !== value) {
      this.searchStringSubject.next(value);
      let searchParams: SearchParamsDTO = {
        FilePath: value,
        NumResults: 300,
        QueryType: "Fuzzy",
      };

      let streamParams: StreamingSearchParamsDTO = {
        StreamIdentifier: "search",
        StartingSize: 10,
        NumEvents: 10,
        Params: searchParams,
      };
      this.searchService.query(streamParams);
    }
  }
}
