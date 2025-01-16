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

  openMenu(menu: ContextMenuComponent, event: MouseEvent, caller: FileModel, state?: FileState) {
    event.preventDefault();

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
    const content = [pin, "div", openInExplorer];

    menu.content = content;
    menu.toggleOpen(event);
  }
}
