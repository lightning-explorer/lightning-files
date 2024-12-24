import { Component, Input, NgZone, OnDestroy, OnInit } from "@angular/core";
import { FormControl, FormsModule, ReactiveFormsModule } from "@angular/forms";
import { debounceTime, Subscription } from "rxjs";
import { SearchParamsDTO } from "@core/dtos/search-params-dto";
import { CommonModule } from "@angular/common";
import { FileResultComponent } from "../../file-result/file-result.component";
import { FileModel, newDefaultFileModel } from "@core/models/file-model";

import { LocalSearchEngineService } from "@core/services/search/text/local-search-engine.service";
import { VectorSearchEngineService } from "@core/services/search/vector/vector-search.service";
import { VectorSearchParamsModel } from "@core/services/search/vector/dtos/output/vector-search-params";

import { vectorResultToModel } from "@core/models/converters/VectorResultToModel";
import { LocalStreamingSearchService } from "@core/services/search/text/local-streaming-search.service";
import { StreamingSearchParamsDTO } from "@core/dtos/streaming-search-params-dtos";
import { HomePageService, SubPage } from "../../../services/home-page.service";
import { ExtendedSearchService } from "../../../pages/extended-search/extended-search.service";
import { DirectoryNavigatorService } from "../../../services/directory-navigator.service";
import { FileOperationsService } from "../../../services/file-operations.service";

@Component({
  selector: "app-searchbar",
  standalone: true,
  imports: [
    ReactiveFormsModule,
    CommonModule,
    FileResultComponent,
    FormsModule,
  ],
  templateUrl: "./searchbar.component.html",
  styleUrl: "./searchbar.component.scss",
})
export class SearchbarComponent implements OnInit, OnDestroy {
  subscription = new Subscription();

  isOnExtendedSearchPage = false;

  exceededSearchResults = false;
  maxSearchResults = 50;
  searchInput: string = "";

  searchResults: FileModel[] = [];
  inputControl = new FormControl();

  constructor(
    private homePageService: HomePageService,
    private extendedSearchService: ExtendedSearchService,

    private searchEngineService: LocalStreamingSearchService,
    private fileOperationsService: FileOperationsService,
    private zone: NgZone // Allows forced change detection
  ) {}

  ngOnInit(): void {
    this.subscription.add(
      this.homePageService.page$.subscribe((page) => {
        this.isOnExtendedSearchPage = page == "extendedSearch";
      })
    );

    this.subscription.add(
      this.inputControl.valueChanges
        .pipe(debounceTime(100))
        .subscribe(async (value) => {
          await this.search(value);
        })
    );

    this.subscription.add(
      this.searchEngineService.files$.subscribe((newFiles) => {
        if (!this.isOnExtendedSearchPage) {
          this.zone.run(() => {
            // Tell the component to update itself
            this.searchResults = newFiles.slice(0, this.maxSearchResults);
            this.exceededSearchResults =
              newFiles.length > this.maxSearchResults;
          });
        }
      })
    );
  }

  ngOnDestroy(): void {
    this.subscription.unsubscribe();
  }

  async search(value: string) {
    let searchParams: SearchParamsDTO = {
      FilePath: value,
      NumResults: this.isOnExtendedSearchPage
        ? 500
        : this.maxSearchResults + 10,
      QueryType: "Fuzzy",
    };

    let streamParams: StreamingSearchParamsDTO = {
      StreamIdentifier: "search",
      StartingSize: 10,
      NumEvents: 10,
      Params: searchParams,
    };
    this.searchEngineService.query(streamParams);
  }

  onResultClick(model: FileModel) {
    //this.inlineSearchService.clearQuery();
    this.fileOperationsService.openOrNavigateToFile(model);
  }

  onBlur() {
    // This doesn't cause any rendering issues
    setTimeout(() => {
      this.searchResults.length = 0;
    }, 100);
  }

  onViewAllResultsClick() {
    this.homePageService.setPage("extendedSearch");
    this.searchResults.length = 0;
    this.extendedSearchService.search(this.searchInput);
  }
}
