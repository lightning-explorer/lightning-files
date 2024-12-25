import { Injectable, OnDestroy, OnInit } from "@angular/core";
import { SearchParamsDTO } from "@core/dtos/search-params-dto";
import { StreamingSearchParamsDTO } from "@core/dtos/streaming-search-params-dtos";
import { FileModel } from "@core/models/file-model";
import { BehaviorSubject, last, Subscription } from "rxjs";
import { HomePageSearchService } from "../../services/home-page-search.service";

@Injectable()
export class ExtendedSearchService {
  searchString$ = this.searchService.searchQueryStr$;
  files$ = this.searchService.files$;

  constructor(private searchService: HomePageSearchService) {}
}
