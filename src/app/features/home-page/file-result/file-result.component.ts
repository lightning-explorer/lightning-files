import { ChangeDetectorRef, Component, Input, OnChanges, SimpleChanges } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatIconModule } from '@angular/material/icon'
import { IconifyIconModule } from '@shared/components/IconifyIcons/icon.module';
import { getIconFromPath } from '@core/util/get-icon-from-path';
import { FileModel } from '@core/models/file-model';
import { HighlightableLabelComponent } from "@shared/components/highlightable-label/highlightable-label.component";
import { PinService } from '@core/services/files/tools/pin.service';
import { FadeInComponent } from "../../../shared/components/misc/fade-in/fade-in.component";

// If you are looking for the drag functionality, it gets handled by the parent component
// 'files-display' for example

@Component({
  selector: 'app-file-result',
  standalone: true,
  imports: [CommonModule, MatIconModule, IconifyIconModule, HighlightableLabelComponent, FadeInComponent],
  templateUrl: './file-result.component.html',
  styleUrl: './file-result.component.scss'
})
export class FileResultComponent {
  isVisible = false;

  @Input() fadeInAnim: boolean = true;

  @Input() file: FileModel | undefined;
  @Input() selected = false;
  @Input() clickEvent: (() => void) | undefined;

  constructor(
    private pinService: PinService
  ) { }

  get isPinned(): boolean {
    if (!this.file)
      return false;
    return this.pinService.isFilePinned(this.file);
  }

  get icon(): string {
    return getIconFromPath(this.file ? this.file.FilePath : "");
  }

  onClick() {
    if (this.clickEvent)
      this.clickEvent();
  }
}
