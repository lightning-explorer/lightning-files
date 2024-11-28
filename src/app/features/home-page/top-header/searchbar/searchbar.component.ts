import { Component, Input, OnInit } from '@angular/core';
import { FormControl, ReactiveFormsModule } from '@angular/forms';
import { debounceTime } from 'rxjs';
import { SearchParamsDTO } from '../../../../core/dtos/output/search-params-dto';
import { CommonModule } from '@angular/common';
import { FileResultComponent } from "../../file-result/file-result.component";
import { FileModel } from '../../../../core/models/file-model';

import { LocalSearchEngineService } from '../../../../core/services/search/text/local-search-engine.service';
import { VectorSearchEngineService } from '../../../../core/services/search/vector/vector-search.service';
import { VectorSearchParamsModel } from '../../../../core/services/search/vector/dtos/output/vector-search-params';

import { DirectoryNavigatorService } from '../../../../core/services/files/directory-navigator/directory-navigator.service';
import { vectorResultToModel } from '../../../../core/models/converters/VectorResultToModel';
import { InlineSearchService } from '../../../../core/services/search/text/inline-search.service';

@Component({
  selector: 'app-searchbar',
  standalone: true,
  imports: [ReactiveFormsModule, CommonModule, FileResultComponent],
  templateUrl: './searchbar.component.html',
  styleUrl: './searchbar.component.scss'
})
export class SearchbarComponent implements OnInit {

  searchResults: FileModel[] = [];
  inputControl = new FormControl();

  constructor(
    private searchEngineService: LocalSearchEngineService,
    private vectorSearchService: VectorSearchEngineService,
    private directoryNavService: DirectoryNavigatorService,
    private inlineSearchService: InlineSearchService
  ) { }

  ngOnInit(): void {

    this.inputControl.valueChanges.pipe(
      debounceTime(100)
    ).subscribe(value =>
      this.search(value)
    )
  }

  async vectorSearch(value: string) {
    let params: VectorSearchParamsModel = { Query: value, Collection: 'files' };
    console.log("vector query");
    let results = await this.vectorSearchService.query(params);
    console.log(results);
    console.log("vector query finished");
    this.searchResults = results.map(x => vectorResultToModel(x));
  }

  async search(value: string) {
    let searchParams: SearchParamsDTO = {
      FilePath: value
    }
    let results = await this.searchEngineService.query(searchParams);
    this.searchResults = results;
  }

  onResultClick(model: FileModel) {
    return () => {
      this.inlineSearchService.clearQuery();
      this.directoryNavService.setCurrentDir(model.FilePath);
    };
  }

  onBlur() {
    setTimeout(() => {
      this.searchResults.length = 0;
    }, 100)

  }

}
