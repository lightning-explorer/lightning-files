import { Injectable } from "@angular/core";
import { FileModel } from "@core/models/file-model";
import { getIconFromPath } from "@core/util/get-icon-from-path";
import { FileViewType } from "./enums/view-type";

@Injectable({ providedIn: "root" })
export class FileResultPresentationService {
  constructor() {}

  getIcon(file:FileModel): string {
    if (file.IsDirectory) return "folder";
    return getIconFromPath(file.FilePath);
  }

  getIconSize(viewType:FileViewType):string{
    switch (viewType) {
        case FileViewType.MediumIcon:
          return "2rem";
        default:
          return "1.2rem";
      }
  }

  isIconType(viewType:FileViewType):boolean{
    switch (viewType) {
      case FileViewType.MediumIcon:
        return true;
      default:
        return false;
    }
  }
}
