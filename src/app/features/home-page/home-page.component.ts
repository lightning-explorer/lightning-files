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
import { CurrentDirectoryBarComponent } from "./current-directory-bar/current-directory-bar.component";
import { TestHtmlComponent } from "../../shared/test-html/test-html.component";
import { FilesDisplayComponent } from "./files-display/files-display.component";

@Component({
  selector: 'app-home-page',
  standalone: true,
  imports: [SearchbarComponent, FileResultComponent, CommonModule, SidebarComponent, CurrentDirectoryBarComponent, TestHtmlComponent, FilesDisplayComponent],
  templateUrl: './home-page.component.html',
  styleUrl: './home-page.component.scss'
})
export class HomePageComponent implements OnInit {

  inputControl = new FormControl();

  searchResults: FileDTOReceived[] = [];
  driveFiles: FileDTOReceived[] = [];

  constructor(private searchEngineService: SearchEngineService) { }

  ngOnInit(): void {
    
  }

  async search(value: string) {
    let searchParams: SearchParamsDTO = {
      FilePath: value
    }
    let results = await this.searchEngineService.query(searchParams);
    console.log(results)
    this.searchResults = results;
  }

}
