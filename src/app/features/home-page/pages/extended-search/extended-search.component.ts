import { Component, NgZone, OnDestroy, OnInit } from "@angular/core";
import { FileModel } from "@core/models/file-model";
import { CommonModule } from "@angular/common";
import { FileResultComponent } from "../../components/file-result/file-result.component";
import { Subscription } from "rxjs";
import { LocalStreamingSearchService } from "@core/services/search/text/local-streaming-search.service";
import { ExtendedSearchService } from "./extended-search.service";
import { FileBrowserComponent } from "../../components/files-display/components/file-browser/file-browser.component";
import { FilesListService } from "../../components/files-display/services/files-list.service";

@Component({
  selector: "app-extended-search",
  standalone: true,
  imports: [CommonModule, FileResultComponent, FileBrowserComponent],
  providers: [FilesListService],
  templateUrl: "./extended-search.component.html",
  styleUrl: "./extended-search.component.css",
})
export class ExtendedSearchComponent implements OnInit, OnDestroy {
  subscription = new Subscription();
  searchQuery$ = this.searchService.searchString$;

  constructor(
    private searchService: ExtendedSearchService,
    private filesListService: FilesListService,
    private zone: NgZone
  ) {

  }

  ngOnInit(): void {
    this.subscription.add(
      this.searchService.files$.subscribe((files) => {
        this.zone.run(() => {
          this.filesListService.setFiles(files);
        });
      })
    );
  }

  ngOnDestroy(): void {
    this.subscription.unsubscribe();
  }
}
