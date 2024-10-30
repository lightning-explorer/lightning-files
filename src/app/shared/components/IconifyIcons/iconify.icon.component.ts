import { Component, Input, OnChanges, OnInit, SimpleChanges } from '@angular/core';
import { IconService } from './icon.service';
import { SafeHtmlPipe } from './safehtml.pipe';

@Component({
    selector: 'iconify-icon',
    template: `<div [innerHTML]="svgIcon | safeHtml"></div>`,
    styles: []
})
export class IconifyIconComponent implements OnInit, OnChanges {
    @Input() icon: string = 'default';
    @Input() size: string | undefined;
    @Input() color: string | undefined;
    svgIcon: string = "";

    constructor(private iconService: IconService) {}

    ngOnInit(): void {
        this.updateIcon();
    }

    ngOnChanges(changes: SimpleChanges): void {
        if (changes['icon'] || changes['size'] || changes['color']) {
            this.updateIcon();
        }
    }

    private updateIcon() {
        this.svgIcon = this.iconService.getIcon(this.icon, this.size, this.color);
    }
}
