import { Injectable } from '@angular/core';
import { ContextMenuComponent } from '../../../../shared/components/context-menu/context-menu.component';
import { ContextMenuButton } from '../../../../shared/components/context-menu/models/ContextMenuButton';
import { FileModel } from '../../models/FileModel';
import { PinService } from '../../../../core/services/files/pin.service';

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