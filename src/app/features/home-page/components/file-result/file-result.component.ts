import { Component, Input, OnInit, ViewChild } from "@angular/core";
import { FileViewType } from "./enums/view-type";
import { CommonModule } from "@angular/common";
import { MatIconModule } from "@angular/material/icon";
import { IconifyIconModule } from "@shared/components/icons/IconifyIcons/icon.module";
import { FileModel } from "@core/models/file-model";
import { HighlightableLabelComponent } from "@shared/components/highlightable-label/highlightable-label.component";
import { PinService } from "src/app/features/home-page/services/pin.service";
import { FileState } from "./file-state";
import { FileResultPresentationService } from "./file-presentation.service";
import { FileContextMenuService } from "./services/context-menu.service";
import { ContextMenuComponent } from "../../../../shared/components/popups/context-menu/context-menu.component";
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
    ContextMenuComponent
  ],
  templateUrl: "./file-result.component.html",
  styleUrl: "./file-result.component.scss",
  animations: [],
  providers: [FileContextMenuService]
})
export class FileResultComponent implements OnInit {
  _iconSize = "1rem";
  _isIconType = false;
  mouseOver = false;

  @ViewChild("contextMenu") contextMenu!: ContextMenuComponent;

  get shouldGrow() {
    if(!this.file)
      return false;
    return (this.file.draggedOver || this.mouseOver) && !this.file.hide;
  }

  @Input() file: FileState | undefined;

  @Input() selected = false;
  @Input() displayPath = false;
  @Input() altColor = false;
  @Input() viewType: FileViewType = FileViewType.Detail;

  constructor(
    private pinService: PinService,
    private presentionService: FileResultPresentationService,
    private contextMenuServce: FileContextMenuService
  ) { }

  ngOnInit(): void {
    this._iconSize = this.presentionService.getIconSize(this.viewType);
    this._isIconType = this.presentionService.isIconType(this.viewType);
  }

  get isPinned(): boolean {
    if (!this.file) return false;
    return this.pinService.isFilePinned(this.file.model);
  }

  getIcon(): string {
    if (this.file)
      return this.presentionService.getIcon(this.file.model);
    return "";
  }

  onRightClick(event: MouseEvent) {
    if (this.file)
      this.contextMenuServce.openMenu(this.contextMenu, event, this.file);
  }

  onMouseEnter() {
    this.mouseOver = true;
  }

  onMouseLeave() {
    this.mouseOver = false;
  }
}
