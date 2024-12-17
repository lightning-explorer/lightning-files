import { Injectable } from '@angular/core';
import { ContextMenuComponent } from '@shared/components/popups/context-menu/context-menu.component';
import { FileModel } from '@core/models/file-model';
import { PinService } from '@core/services/files/tools/pin.service';

@Injectable()
export class FileContextMenuService {

    constructor(private pinService: PinService) { }

    openMenu(menu: ContextMenuComponent, event: MouseEvent, caller: FileModel) {
        event.preventDefault();

        const pin = this.pinService.isFilePinned(caller) ? {
            name: 'Unpin', action: () => {
                this.pinService.unpinFile(caller);
            }
        } : {
            name: 'Pin', action: () => {
                this.pinService.pinFile(caller);
            }
        };
        const buttons = [pin];

        menu.buttons = buttons;
        menu.toggleOpen(event);
    }

}