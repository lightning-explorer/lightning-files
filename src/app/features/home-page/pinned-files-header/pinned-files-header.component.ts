import { Component, ViewChild } from '@angular/core';
import { PinService } from '../../../core/services/files/pin.service';
import { FileModel } from '../../../core/models/file-model';
import { CommonModule } from '@angular/common';
import { FileResultComponent } from "../file-result/file-result.component";
import { DirectoryNavigatorService } from '../../../core/services/files/directory-navigator/directory-navigator.service';
import { CssVarToHexService } from '../../../core/services/misc/css-var-to-hex.service';
import { getIconFromPath } from '../../../core/other/util/get-icon-from-path';
import { IconifyIconModule } from '../../../shared/components/IconifyIcons/icon.module';
import { truncateText } from '../../../core/other/util/text-truncator';
import { SelectService } from '../files-display/services/select.service';
import { ContextMenuComponent } from "../../../shared/components/context-menu/context-menu.component";
import { FileContextMenuService } from '../files-display/services/context-menu.service';

@Component({
  selector: 'app-pinned-files-header',
  standalone: true,
  imports: [CommonModule, FileResultComponent, IconifyIconModule, ContextMenuComponent],
  providers: [SelectService, FileContextMenuService],
  templateUrl: './pinned-files-header.component.html',
  styleUrl: './pinned-files-header.component.css'
})
export class PinnedFilesHeaderComponent {
  @ViewChild('contextMenu') contextMenu!: ContextMenuComponent;
  pinnedFiles: FileModel[] = [];

  constructor(private pinService: PinService, private selectService: SelectService,
    private contextMenuService: FileContextMenuService) {
    this.pinService.pinnedFiles$.subscribe(x => this.pinnedFiles = x);
  }

  onFileClick(file: FileModel) {
    this.selectService.onFileDoubleClick(file);
  }

  onFileRightClick(event: MouseEvent, file: FileModel) {
    this.contextMenuService.openMenu(this.contextMenu, event, file);
  }

  processFilename(name: string) {
    return truncateText(name, 14);
  }

  getIcon(file: FileModel): string {
    return getIconFromPath(file ? file.Dto.FilePath : "");
  }

}
