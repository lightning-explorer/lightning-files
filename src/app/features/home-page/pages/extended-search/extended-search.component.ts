import { Component, NgZone, OnDestroy, OnInit } from "@angular/core";
import { CommonModule } from "@angular/common";
import { Subscription } from "rxjs";
import { FileBrowserComponent } from "../../components/files-display/components/file-browser/file-browser.component";
import { FilesListService } from "../../components/files-display/services/files-list.service";
import { HomePageSearchService } from "../../services/home-page-search.service";

@Component({
  selector: "app-extended-search",
  standalone: true,
  imports: [CommonModule, FileBrowserComponent],
  providers: [FilesListService],
  templateUrl: "./extended-search.component.html",
  styleUrl: "./extended-search.component.css",
})
export class ExtendedSearchComponent implements OnInit, OnDestroy {
  subscription = new Subscription();
  searchQuery$ = this.searchService.searchQueryStr$;

  constructor(
    private searchService: HomePageSearchService,
    private filesListService: FilesListService,
    private zone: NgZone
  ) {

  }

  ngOnInit(): void {
    this.subscription.add(
      this.searchService.files$.subscribe((files) => {
        this.zone.run(() => {
          this.filesListService.setFilesDefault(files);
        });
      })
    );
  }

  ngOnDestroy(): void {
    this.subscription.unsubscribe();
  }
}
