import { Injectable } from "@angular/core";
import { ContextMenuComponent } from "@shared/components/popups/context-menu/context-menu.component";
import { FileModel } from "@core/models/file-model";
import { PinService } from "src/app/features/home-page/services/pin.service";
import { TauriCommandsService } from "@core/services/tauri/commands.service";
import { FileState } from "../file-state";

@Injectable()
export class FileContextMenuService {
  constructor(
    private pinService: PinService,
    private commandsService: TauriCommandsService
  ) {}

  openMenu(menu: ContextMenuComponent, event: MouseEvent, file: FileState) {
    event.preventDefault();
    const caller = file.model;

    const pin = this.pinService.isFilePinned(caller)
      ? {
          name: "Unpin",
          action: () => {
            this.pinService.unpinFile(caller);
          },
        }
      : {
          name: "Quick Pin",
          action: () => {
            this.pinService.pinFile(caller);
          },
        };
    const openInExplorer = {
      name: "Open in Explorer",
      action: () => {
        this.commandsService.openInExplorer(caller.FilePath);
      },
    };
    const rename = {
      name: "Rename",
      action: () => {
        file = {...file, renameRequested:true};
      }
    }
    const content = [pin, "div", openInExplorer, rename];

    menu.content = content;
    menu.toggleOpen(event);
  }
}
