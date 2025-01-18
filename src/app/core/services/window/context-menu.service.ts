import { ComponentRef, Injectable } from '@angular/core';
import { ContextMenuButton } from '@shared/components/popups/context-menu/models/ContextMenuButton';
import { ComponentCreatorService } from './component-creator.service';
import { ContextMenuComponent, ContextMenuItem } from '@shared/components/popups/context-menu/context-menu.component';
import { Observable, Subscription } from 'rxjs';

@Injectable({providedIn: 'root'})
export class ContextMenuService {
    private subscription = new Subscription();
    private menu?: ComponentRef<ContextMenuComponent>;

    constructor(private componentCreatorService:ComponentCreatorService) { }

    open(event:MouseEvent, buttons:ContextMenuItem[]){
        const menu = this.componentCreatorService.addToDocumentBody(ContextMenuComponent);

        const closeObservable = menu.instance.close.asObservable();
        this.subscription.add(closeObservable.subscribe(_=>{
            console.log("closeing");
            this.close();
        }));

        menu.instance.content = buttons;
        menu.instance.toggleOpen(event);
        this.menu = menu;
    }

    close(){
        if(this.menu){
            this.subscription.unsubscribe();
            this.componentCreatorService.removeFromDocumentBody(this.menu);
        }
    }
    
}