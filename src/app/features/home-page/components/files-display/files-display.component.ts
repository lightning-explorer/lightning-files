import { Component, OnDestroy, OnInit } from "@angular/core";
import { FileBrowserComponent } from "./components/file-browser/file-browser.component";
import { FormControl } from "@angular/forms";
import { FileModel } from "@core/models/file-model";
import { FilePreviewComponent } from "./components/file-preview/file-preview.component";
import { Observable, Subscription } from "rxjs";
import { FilesListService } from "./services/files-list.service";
import { CommonModule } from "@angular/common";
import { DirectoryNavigatorService } from "../../services/directory-navigator.service";
import { DirectoryWatcherService } from "../../services/directory-watcher.service";

@Component({
  selector: "app-files-display",
  standalone: true,
  imports: [FileBrowserComponent, CommonModule],
  providers: [FilesListService],
  templateUrl: "./files-display.component.html",
  styleUrl: "./files-display.component.scss",
})
export class FilesDisplayComponent implements OnInit, OnDestroy {
  private subscription = new Subscription();

  isLoading = false;
  inputControl = new FormControl();
  noFilesMsg = "";

  // The file that the user last selected on (clicked)
  previewFile: FileModel | undefined;

  constructor(
    private filesListService: FilesListService,
    private directoryService: DirectoryNavigatorService,
    // Is this the correct place to put this service?
    private watcherService: DirectoryWatcherService
  ) {}

  async ngOnInit() {
    this.subscription.add(
      this.directoryService.currentFiles$.subscribe((files) =>
        this.filesListService.setFiles(files)
      )
    );
    this.subscription.add(
      this.directoryService.isLoading$.subscribe((x) => (this.isLoading = x))
    );
    this.subscription.add(
      this.directoryService.currentDir$.subscribe(async (dir) => {
        this.noFilesMsg = dir;
        this.watcherService.watchDirectory(dir);
      })
    );
    this.subscription.add(
      this.watcherService.directoryChanges$.subscribe(() => {
        this.directoryService.setFiles();
      })
    );
    this.directoryService.setFiles();
  }

  ngOnDestroy(): void {
    this.subscription.unsubscribe();
  }
}
