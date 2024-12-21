import { Component, OnDestroy, OnInit } from "@angular/core";
import { FileBrowserComponent } from "./components/file-browser/file-browser.component";
import { DirectoryNavigatorService } from "@core/services/files/directory-navigator/directory-navigator.service";
import { FormControl } from "@angular/forms";
import { FileModel } from "@core/models/file-model";
import { FilePreviewComponent } from "./components/file-preview/file-preview.component";
import { Subscription } from "rxjs";
import { FilesListService } from "./files-list.service";
import { CommonModule } from "@angular/common";

@Component({
  selector: "app-files-display",
  standalone: true,
  imports: [FileBrowserComponent, CommonModule],
  providers: [FilesListService],
  templateUrl: "./files-display.component.html",
  styleUrl: "./files-display.component.scss",
})
export class FilesDisplayComponent implements OnInit {

  isLoading$ = this.directoryService.isLoading$;
  inputControl = new FormControl();

  // The file that the user last selected on (clicked)
  previewFile: FileModel | undefined;

  constructor(
    private directoryService: DirectoryNavigatorService,
  ) {}

  ngOnInit(): void {
    this.directoryService.setDriveFiles();
  }

}
