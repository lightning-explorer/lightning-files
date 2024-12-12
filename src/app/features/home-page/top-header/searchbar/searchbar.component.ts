import { Component, Input, NgZone, OnDestroy, OnInit } from '@angular/core';
import { FormControl, FormsModule, ReactiveFormsModule } from '@angular/forms';
import { debounceTime, Subscription } from 'rxjs';
import { SearchParamsDTO } from '../../../../core/dtos/output/search-params-dto';
import { CommonModule } from '@angular/common';
import { FileResultComponent } from "../../file-result/file-result.component";
import { FileModel, newDefaultFileModel } from '../../../../core/models/file-model';

import { LocalSearchEngineService } from '../../../../core/services/search/text/local-search-engine.service';
import { VectorSearchEngineService } from '../../../../core/services/search/vector/vector-search.service';
import { VectorSearchParamsModel } from '../../../../core/services/search/vector/dtos/output/vector-search-params';

import { DirectoryNavigatorService } from '../../../../core/services/files/directory-navigator/directory-navigator.service';
import { vectorResultToModel } from '../../../../core/models/converters/VectorResultToModel';
import { InlineSearchService } from '../../../../core/services/search/text/inline-search.service';
import { LocalStreamingSearchService } from '../../../../core/services/search/text/local-streaming-search.service';
import { StreamingSearchParamsDTO } from '../../../../core/dtos/output/streaming-search-params-dtos';
import { HomePageService, SubPage } from '../../services/home-page.service';
import { ExtendedSearchService } from '../../services/extended-search.service';

@Component({
  selector: 'app-searchbar',
  standalone: true,
  imports: [ReactiveFormsModule, CommonModule, FileResultComponent, FormsModule],
  templateUrl: './searchbar.component.html',
  styleUrl: './searchbar.component.scss'
})
export class SearchbarComponent implements OnInit, OnDestroy {
  subscription = new Subscription();

  isOnExtendedSearchPage = false;

  exceededSearchResults = false;
  maxSearchResults = 50;
  searchInput: string = '';

  searchResults: FileModel[] = [];
  inputControl = new FormControl();

  constructor(
    private homePageService: HomePageService,
    private extendedSearchService: ExtendedSearchService,

    private searchEngineService: LocalStreamingSearchService,
    private directoryNavService: DirectoryNavigatorService,
    private inlineSearchService: InlineSearchService,
    private zone: NgZone // Allows forced change detection
  ) { }

  ngOnInit(): void {

    this.subscription.add(this.homePageService.page$.subscribe(page => {
      this.isOnExtendedSearchPage = page == 'extended-search';
    }));

    this.subscription.add(this.inputControl.valueChanges.pipe(
      debounceTime(100)
    ).subscribe(async value => {
      await this.search(value)
    }));

    this.subscription.add(this.searchEngineService.files$.subscribe(newFiles => {
      if (!this.isOnExtendedSearchPage) {
        this.zone.run(() => { // Tell the component to update itself
          this.searchResults = newFiles.slice(0, this.maxSearchResults);
          this.exceededSearchResults = newFiles.length > this.maxSearchResults;
        })
      }
    }));

  }

  ngOnDestroy(): void {
    this.subscription.unsubscribe();
  }

  async search(value: string) {

    let searchParams: SearchParamsDTO = {
      FilePath: value,
      NumResults: this.isOnExtendedSearchPage ? 500 : this.maxSearchResults + 10
    }

    let streamParams: StreamingSearchParamsDTO = {
      StreamIdentifier: "search",
      StartingSize: 10,
      NumEvents: 10,
      Params: searchParams,
    }
    this.searchEngineService.query(streamParams);

  }

  onResultClick(model: FileModel) {
    return () => {
      this.inlineSearchService.clearQuery();
      this.directoryNavService.setCurrentDir(model.FilePath);
    };
  }

  onBlur() {
    // This doesn't cause any rendering issues
    setTimeout(() => {
      this.searchResults.length = 0;
    }, 100)
  }

  onViewAllResultsClick() {
    this.homePageService.setPage('extended-search');
    this.extendedSearchService.search(this.searchInput);
  }
}
