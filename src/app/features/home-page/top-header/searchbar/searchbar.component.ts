import { Component, Input, NgZone, OnDestroy, OnInit } from '@angular/core';
import { FormControl, ReactiveFormsModule } from '@angular/forms';
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

@Component({
  selector: 'app-searchbar',
  standalone: true,
  imports: [ReactiveFormsModule, CommonModule, FileResultComponent],
  templateUrl: './searchbar.component.html',
  styleUrl: './searchbar.component.scss'
})
export class SearchbarComponent implements OnInit, OnDestroy {
  subscription = new Subscription();

  exceededSearchResults = false;
  maxSearchResults = 50;

  searchResults: FileModel[] = [];
  inputControl = new FormControl();

  constructor(
    private searchEngineService: LocalStreamingSearchService,
    private directoryNavService: DirectoryNavigatorService,
    private inlineSearchService: InlineSearchService,
    private zone: NgZone // Allows forced change detection
  ) { }

  ngOnInit(): void {

    this.subscription.add(this.inputControl.valueChanges.pipe(
      debounceTime(100)
    ).subscribe(async value =>
      await this.search(value)
    ));

    this.subscription.add(this.searchEngineService.files$.subscribe(newFiles => {
      this.zone.run(() => { // Tell the component to update itself
        this.searchResults = newFiles.slice(0, this.maxSearchResults);
        console.log(newFiles.length);
        this.exceededSearchResults = newFiles.length > this.maxSearchResults;
      })
    }));

  }

  ngOnDestroy(): void {
    this.subscription.unsubscribe();
  }

  async search(value: string) {

    let searchParams: SearchParamsDTO = {
      FilePath: value,
      NumResults: this.maxSearchResults + 10
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

}
