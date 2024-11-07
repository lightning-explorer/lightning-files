import { ChangeDetectorRef, Component, Input, OnChanges, SimpleChanges } from '@angular/core';
import { FileDTOReceived } from '../../../core/dtos/file-dto-received';
import { CommonModule } from '@angular/common';
import { MatIconModule } from '@angular/material/icon'
import { DirectoryNavigatorService } from '../../../core/services/files/directory-navigator/directory-navigator.service';
import { IconifyIconModule } from '../../../shared/components/IconifyIcons/icon.module';
import { getIconFromPath } from '../../../core/other/util/get-icon-from-path';
import { CssVarToHexService } from '../../../core/services/misc/css-var-to-hex.service';
import { FileModel } from '../models/FileModel';
import { HighlightableLabelComponent } from "../../../shared/components/highlightable-label/highlightable-label.component";
import { PinService } from '../../../core/services/files/pin.service';

@Component({
  selector: 'app-file-result',
  standalone: true,
  imports: [CommonModule, MatIconModule, IconifyIconModule, HighlightableLabelComponent],
  templateUrl: './file-result.component.html',
  styleUrl: './file-result.component.scss'
})
export class FileResultComponent {
  @Input() file: FileModel | undefined;
  @Input() selected = false;

  constructor(private directoryService: DirectoryNavigatorService,
    private cssVarService: CssVarToHexService,
    private pinService: PinService
  ) { }

  get isPinned(): boolean {
    if (!this.file)
      return false;
    return this.pinService.isFilePinned(this.file);
  }

  get icon(): string {
    return getIconFromPath(this.file ? this.file.Dto.FilePath : "");
  }

  get iconColor(): string {
    return this.cssVarService.cssVarToHex('--secondary-color');
  }
}
