import { Component, Input } from "@angular/core";
import { IconifyIconModule } from "../../../../shared/components/icons/IconifyIcons/icon.module";
import { FileViewType } from "../file-result/enums/view-type";
import { getIconFromPath } from "@core/util/get-icon-from-path";

@Component({
  selector: "app-file-icon",
  standalone: true,
  imports: [IconifyIconModule],
  templateUrl: "./file-icon.component.html",
  styleUrl: "./file-icon.component.css",
})
export class FileIconComponent {
  @Input() filePath?: string;
  @Input() isDirectory = false;
  @Input() viewType: FileViewType = FileViewType.Detail;

   get icon(): string {
    if (this.filePath) {
      if (this.isDirectory) return "folder";
      return getIconFromPath(this.filePath);
    }
    return "";
  }

  get iconSize(): string {
    switch (this.viewType) {
      case FileViewType.MediumIcon:
        return "2rem";
      default:
        return "1.2rem";
    }
  }

  isIconType(): boolean {
    switch (this.viewType) {
      case FileViewType.MediumIcon:
        return true;
      default:
        return false;
    }
  }
}
