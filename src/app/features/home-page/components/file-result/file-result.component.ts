import {
  ChangeDetectorRef,
  Component,
  Input,
  OnChanges,
  SimpleChanges,
} from "@angular/core";
import { CommonModule } from "@angular/common";
import { MatIconModule } from "@angular/material/icon";
import { IconifyIconModule } from "@shared/components/icons/IconifyIcons/icon.module";
import { getIconFromPath } from "@core/util/get-icon-from-path";
import { FileModel } from "@core/models/file-model";
import { HighlightableLabelComponent } from "@shared/components/highlightable-label/highlightable-label.component";
import { PinService } from "src/app/features/home-page/services/pin.service";
import { FadeInComponent } from "@shared/components/misc/fade-in/fade-in.component";
import { defaultFileState, FileState } from "./file-state";
import { FilesListService } from "../files-display/services/files-list.service";
import { DirectoryNavigatorService } from "../../services/directory-navigator.service";
import { IconifyIconClusterComponent } from "../../../../shared/components/icons/iconify-icon-cluster/iconify-icon-cluster.component";

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
    FadeInComponent,
    IconifyIconClusterComponent
],
  templateUrl: "./file-result.component.html",
  styleUrl: "./file-result.component.scss",
})
export class FileResultComponent {
  mouseOver = false;

  get shouldGrow() {
    return (this.state.draggedOver || this.mouseOver) && !this.state.hide;
  }

  @Input() fadeInAnim: boolean = true;

  @Input() file: FileModel | undefined;
  @Input() state: FileState = defaultFileState();
  @Input() selected = false;

  constructor(
    private pinService: PinService,
    private directoryNavService: DirectoryNavigatorService
  ) {}

  get isPinned(): boolean {
    if (!this.file) return false;
    return this.pinService.isFilePinned(this.file);
  }

  get icon(): string {
    return getIconFromPath(this.file ? this.file.FilePath : "");
  }

  onClick() {}

  onMouseEnter() {
    this.mouseOver = true;
  }

  onMouseLeave() {
    this.mouseOver = false;
  }
}
