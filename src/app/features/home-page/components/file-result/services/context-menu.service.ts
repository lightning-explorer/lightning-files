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

  openMenu(
    menu: ContextMenuComponent,
    event: MouseEvent,
    caller: FileModel,
    state?: FileState
  ) {
    event.preventDefault();
    const caller = file.model;

    let content: any[] = [];
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
    content.push(pin);
    const openInExplorer = {
      name: "Open in Explorer",
      action: () => {
        this.commandsService.openInExplorer(caller.FilePath);
      },
    };
    content.push(openInExplorer);
    if (state) {
      const rename = {
        name: "Rename",
        action: () => {
          state.requestRename=true;
        },
      };
      content.push(rename);
    }

    menu.content = content;
    menu.toggleOpen(event);
  }
}
