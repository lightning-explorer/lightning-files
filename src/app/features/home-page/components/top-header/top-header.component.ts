import { Component } from "@angular/core";
import { CurrentDirectoryBarComponent } from "./current-directory-bar/current-directory-bar.component";
import { MatIconModule } from "@angular/material/icon";
import { SearchbarComponent } from "./searchbar/searchbar.component";
import { DirectoryHistoryService } from "src/app/features/home-page/services/directory-history.service";
import { DirectoryNavigatorService } from "../../services/directory-navigator.service";
import { PinnedFilesHeaderComponent } from "./pinned-files-header/pinned-files-header.component";
import { TabsHolderComponent } from "./tabs-holder/tabs-holder.component";

@Component({
  selector: "app-top-header",
  standalone: true,
  imports: [
    CurrentDirectoryBarComponent,
    SearchbarComponent,
    MatIconModule,
    PinnedFilesHeaderComponent,
    TabsHolderComponent
],
  templateUrl: "./top-header.component.html",
  styleUrl: "./top-header.component.css",
})
export class TopHeaderComponent {
  constructor(
    private directoryService: DirectoryNavigatorService,
    private directoryHistoryService: DirectoryHistoryService
  ) {}

  async onNavigateBackDirectoryClick() {
    let parent = await this.directoryService.getParentDirectory();
    await this.directoryService.setCurrentDir(parent);
  }

  async onUndoClick() {
    this.directoryHistoryService.undo();
  }

  async onRedoClick() {
    this.directoryHistoryService.redo();
  }
}
