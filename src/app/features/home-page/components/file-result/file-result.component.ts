import { Component, Input, OnInit } from "@angular/core";
import { FileViewType } from "./enums/view-type";
import { CommonModule } from "@angular/common";
import { MatIconModule } from "@angular/material/icon";
import { IconifyIconModule } from "@shared/components/icons/IconifyIcons/icon.module";
import { FileModel } from "@core/models/file-model";
import { HighlightableLabelComponent } from "@shared/components/highlightable-label/highlightable-label.component";
import { PinService } from "src/app/features/home-page/services/pin.service";
import { defaultFileState, FileState } from "./file-state";
import { FileResultPresentationService } from "./file-presentation.service";
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
  animations: [],
})
export class FileResultComponent {
  mouseOver = false;

  get shouldGrow() {
    return (this.state.draggedOver || this.mouseOver) && !this.state.hide;
  }

  @Input() file: FileModel | undefined;
  @Input() state: FileState = defaultFileState();

  @Input() selected = false;
  @Input() displayPath = false;
  @Input() altColor = false;
  @Input() viewType: FileViewType = FileViewType.Detail;

  constructor(
    private pinService: PinService,
    private presentionService: FileResultPresentationService
  ) {}

  get isPinned(): boolean {
    if (!this.file) return false;
    return this.pinService.isFilePinned(this.file);
  }

  getIcon():string{
    if(this.file)
      return this.presentionService.getIcon(this.file);
    return "";
  }

  getIconSize():string{
    return this.presentionService.getIconSize(this.viewType);
  }

  getCssClass() {
    return this.presentionService.getCssClass(this.viewType);
  }

  getBodyCssClass() {
    return this.presentionService.getBodyCssClass(this.viewType);
  }

  onMouseEnter() {
    this.mouseOver = true;
  }

  onMouseLeave() {
    this.mouseOver = false;
  }
}
