import { Component, Input, OnInit } from '@angular/core';
import { IconService } from './icon.service';
import { SafeHtmlPipe } from './safehtml.pipe';

@Component({
    selector: 'iconify-icon',
    template: `<div [innerHTML]="svgIcon | safeHtml"></div>`,
    styles: []
})
export class IconifyIconComponent implements OnInit {
    @Input() icon: string = 'default';
    @Input() size: string|undefined;
    svgIcon: string = "";

    constructor(private iconService: IconService) { }

    ngOnInit(): void {
        const icon = this.iconService.getIcon(this.icon, this.size);
        this.svgIcon = icon ? icon : "";
    }
}

