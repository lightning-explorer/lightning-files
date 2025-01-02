import { Component, HostBinding, Input, OnChanges, OnInit, SimpleChanges } from "@angular/core";
import { CommonModule } from "@angular/common";
import { MatIconModule } from "@angular/material/icon";
import { IconifyIconModule } from "@shared/components/icons/IconifyIcons/icon.module";
import { getIconFromPath } from "@core/util/get-icon-from-path";
import { FileModel } from "@core/models/file-model";
import { HighlightableLabelComponent } from "@shared/components/highlightable-label/highlightable-label.component";
import { PinService } from "src/app/features/home-page/services/pin.service";
import { defaultFileState, FileState } from "./file-state";
import { fadeInAnimation } from "@shared/animations/fade-in-animation";
// If you are looking for the drag functionality, it gets handled by the parent component
// 'files-display' for example

@Component({
  selector: "app-file-result",
  standalone: true,
  imports: [
    CommonModule,
    MatIconModule,
    IconifyIconModule,
    HighlightableLabelComponent,
],
  templateUrl: "./file-result.component.html",
  styleUrl: "./file-result.component.scss",
})
export class FileResultComponent{
  mouseOver = false;

  get shouldGrow() {
    return (this.state.draggedOver || this.mouseOver) && !this.state.hide;
  }

  @Input() file: FileModel | undefined;
  @Input() state: FileState = defaultFileState();
  @Input() selected = false;
  @Input() displayPath = false;
  @Input() altColor = false;

  constructor(
    private pinService: PinService,
  ) {}

  get isPinned(): boolean {
    if (!this.file) return false;
    return this.pinService.isFilePinned(this.file);
  }
 
  get icon(): string {
    return getIconFromPath(this.file ? this.file.FilePath : "");
  }

  onMouseEnter() {
    this.mouseOver = true;
  }

  onMouseLeave() {
    this.mouseOver = false;
  }

}
