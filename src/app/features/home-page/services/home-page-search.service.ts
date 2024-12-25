import { Injectable, OnDestroy } from "@angular/core";
import { SearchParamsDTO } from "@core/dtos/search-params-dto";
import { LocalStreamingSearchService } from "@core/services/search/text/local-streaming-search.service";
import { HomePageService, SubPage } from "./home-page.service";
import { BehaviorSubject, Subscription } from "rxjs";
import { StreamingSearchParamsDTO } from "@core/dtos/streaming-search-params-dtos";

@Injectable()
export class HomePageSearchService implements OnDestroy {
  private subscription = new Subscription();

  readonly maxSearchResults = 50;
  private isOnExtendedSearchPageSubject = new BehaviorSubject<boolean>(false);
  isOnExtendedSearchPage$ = this.isOnExtendedSearchPageSubject.asObservable();

  private searchQueryStrSubject = new BehaviorSubject<string>("");
  searchQueryStr$ = this.searchQueryStrSubject.asObservable();

  files$ = this.searchService.files$;

  private page: SubPage | undefined;

  constructor(
    private searchService: LocalStreamingSearchService,
    private homePageService: HomePageService
  ) {
    this.subscription.add(
      this.homePageService.page$.subscribe((x) => {
        this.page = x;
        this.isOnExtendedSearchPageSubject.next(x === "extendedSearch");
      })
    );
  }

  async search(params: Partial<SearchParamsDTO>) {
    // Assuming we are still querying based off the file path
    this.searchQueryStrSubject.next(params.FilePath ?? "");

    let searchParams: SearchParamsDTO = {
      NumResults: this.isOnExtendedSearch() ? 500 : this.maxSearchResults,
      QueryType: "Fuzzy",
      ...params,
    };

    let streamParams: StreamingSearchParamsDTO = {
      StreamIdentifier: "search",
      StartingSize: 10,
      NumEvents: 10,
      Params: searchParams,
    };
    await this.searchService.query(streamParams);
  }

  isOnExtendedSearch(): boolean {
    return this.page == "extendedSearch";
  }

  ngOnDestroy(): void {
    this.subscription.unsubscribe();
  }
}
