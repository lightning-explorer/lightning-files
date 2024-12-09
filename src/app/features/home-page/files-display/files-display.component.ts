import { Component, HostListener, OnDestroy } from "@angular/core";
import { FileBrowserComponent } from "./components/file-browser/file-browser.component";
import { DirectoryNavigatorService } from "@core/services/files/directory-navigator/directory-navigator.service";
import { InlineSearchService } from "@core/services/search/text/inline-search.service";
import { FormControl } from "@angular/forms";
import { FileModel } from "@core/models/file-model";
import { FilePreviewComponent } from "./components/file-preview/file-preview.component";
import { Subscription } from "rxjs";


@Component({
  selector: 'app-files-display',
  standalone: true,
  imports: [FileBrowserComponent, FilePreviewComponent],
  providers: [],
  templateUrl: './files-display.component.html',
  styleUrl: './files-display.component.scss',
})
export class FilesDisplayComponent implements OnDestroy {
  subscription = new Subscription();

  inputControl = new FormControl();
  driveFiles: FileModel[] = [];

  loadingFiles = true;

  // The file that the user last selected on (clicked)
  previewFile: FileModel | undefined;

  constructor(
    private directoryService: DirectoryNavigatorService,
    private inlineSearchService: InlineSearchService
  ) { }

  ngOnInit(): void {
    this.directoryService.setDriveFiles();

    this.subscription.add(this.directoryService.isLoading$.subscribe(loading =>
      this.loadingFiles = loading
    ));

    this.subscription.add(this.directoryService.currentFiles$.subscribe(files => {
      return this.driveFiles = files;
    }));
  }

  ngOnDestroy(): void {
    this.subscription.unsubscribe();
  }

  @HostListener('window:keydown', ['$event'])
  async handleKeydown(event: KeyboardEvent) {
    this.inlineSearchService.handleKeydown(event, this.driveFiles);
  }

  onUndoClick() {

  }

}
