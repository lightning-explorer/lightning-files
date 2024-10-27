import { Component, OnInit } from '@angular/core';
import { SearchEngineService } from '../../core/services/search-engine.service';
import { SearchbarComponent } from "./searchbar/searchbar.component";
import { FileDTOReceived } from '../../core/services/dtos/file-dto-received';
import { FileResultComponent } from "./file-result/file-result.component";
import { CommonModule } from '@angular/common';
import { FormControl } from '@angular/forms';
import { debounceTime } from 'rxjs';
import { SearchParamsDTO } from '../../core/services/dtos/search-params-dto';
import { SidebarComponent } from "./sidebar/sidebar.component";

@Component({
  selector: 'app-home-page',
  standalone: true,
  imports: [SearchbarComponent, FileResultComponent, CommonModule, SidebarComponent],
  templateUrl: './home-page.component.html',
  styleUrl: './home-page.component.scss'
})
export class HomePageComponent {

  inputControl = new FormControl();
  searchResults: FileDTOReceived[] = [];

  constructor(private searchEngineService: SearchEngineService) { }

  async search(value: string) {
    let searchParams: SearchParamsDTO = {
      FilePath: value
    }
    let results = await this.searchEngineService.query(searchParams);
    console.log(results)
    this.searchResults = results;
  }

}
