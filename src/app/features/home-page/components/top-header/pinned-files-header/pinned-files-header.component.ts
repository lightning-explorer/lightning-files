import { Component, OnDestroy, ViewChild } from "@angular/core";
import { PinService } from "src/app/features/home-page/services/pin.service";
import { FileModel } from "@core/models/file-model";
import { CommonModule } from "@angular/common";
import { getIconFromPath } from "@core/util/get-icon-from-path";
import { IconifyIconModule } from "@shared/components/icons/IconifyIcons/icon.module";
import { truncateText } from "@core/util/text-truncator";
import { SelectService } from "../../files-display/components/file-browser/services/interaction/select.service";
import { ContextMenuComponent } from "@shared/components/popups/context-menu/context-menu.component";
import { FileContextMenuService } from "../../file-result/services/context-menu.service";
import { Subscription } from "rxjs";
import { ToolbarComponent } from "../../sidebar/toolbar/toolbar.component";
import { TooltipComponent } from "../../../../../shared/components/popups/tooltip/tooltip.component";
import { TooltipDirective } from "@shared/components/popups/tooltip/tooltip.directive";
import { defaultFileState } from "../../file-result/file-state";

@Component({
  selector: "app-pinned-files-header",
  standalone: true,
  imports: [CommonModule, IconifyIconModule, ContextMenuComponent, TooltipDirective],
  providers: [SelectService, FileContextMenuService],
  templateUrl: "./pinned-files-header.component.html",
  styleUrl: "./pinned-files-header.component.css",
})
export class PinnedFilesHeaderComponent implements OnDestroy {
  subscription = new Subscription();
  @ViewChild("contextMenu") contextMenu!: ContextMenuComponent;
  pinnedFiles: FileModel[] = [];

  constructor(
    private pinService: PinService,
    private selectService: SelectService,
    private contextMenuService: FileContextMenuService
  ) {
    this.subscription.add(
      this.pinService.pinnedFiles$.subscribe((x) => (this.pinnedFiles = x))
    );
  }

  ngOnDestroy(): void {
    this.subscription.unsubscribe();
  }

  onFileClick(file: FileModel) {
    this.selectService.onFileDoubleClick(file);
  }

  onFileRightClick(event: MouseEvent, file: FileModel) {
    const state = defaultFileState(file);
    this.contextMenuService.openMenu(this.contextMenu, event, state);
  }

  processFilename(name: string) {
    return truncateText(name, 20);
  }

  getIcon(file: FileModel): string {
    return getIconFromPath(file ? file.FilePath : "");
  }
}
