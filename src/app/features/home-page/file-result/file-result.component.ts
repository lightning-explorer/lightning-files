import { ChangeDetectorRef, Component, Input, OnChanges, SimpleChanges } from '@angular/core';
import { FileDTOReceived } from '../../../core/dtos/file-dto-received';
import { CommonModule } from '@angular/common';
import { MatIconModule } from '@angular/material/icon'
import { DirectoryNavigatorService } from '../../../core/services/files/directory-navigator.service';
import { IconifyIconModule } from '../../../shared/components/IconifyIcons/icon.module';
import { getIconFromPath } from '../../../core/other/util/get-icon-from-path';
import { CssVarToHexService } from '../../../core/services/misc/css-var-to-hex.service';
import { FileModel } from '../models/FileModel';
import { HighlightableLabelComponent } from "../../../shared/components/highlightable-label/highlightable-label.component";

@Component({
  selector: 'app-file-result',
  standalone: true,
  imports: [CommonModule, MatIconModule, IconifyIconModule, HighlightableLabelComponent],
  templateUrl: './file-result.component.html',
  styleUrl: './file-result.component.scss'
})
export class FileResultComponent {
  @Input() file: FileModel | undefined;

  constructor(private directoryService: DirectoryNavigatorService,
    private cssVarService: CssVarToHexService,
  ) { }

  async onClick() {
    if (this.file) {
      console.log('clcu');
      const path = this.file.Dto.FilePath;
      if (await this.directoryService.isPathAFile(path)) {
        await this.directoryService.openFileCmd(path);
      } else {
        await this.directoryService.setCurrentDir(path);
      }
    }
  }

  get icon(): string {
    return getIconFromPath(this.file ? this.file.Dto.FilePath : "");
  }

  get iconColor(): string {
    return this.cssVarService.cssVarToHex('--secondary-color');
  }
}
