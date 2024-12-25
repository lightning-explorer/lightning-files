import { Injectable } from "@angular/core";
import { HomePageSearchService } from "../../services/home-page-search.service";

@Injectable()
export class ExtendedSearchService {
  searchString$ = this.searchService.searchQueryStr$;
  files$ = this.searchService.files$;

  constructor(private searchService: HomePageSearchService) {}
}
 