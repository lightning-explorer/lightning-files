import { Component, Input, OnChanges, OnInit, SimpleChanges } from '@angular/core';
import { IconService } from './icon.service';
import { SafeHtmlPipe } from './safehtml.pipe';

/**
 * Usage example:
 * 
 * `<iconify-icon icon="hardDrive" size="1.2em" color="#fff" />`
 * 
 * You are also able to pass in a CSS variable for the color:
 * 
 * `<iconify-icon icon="hardDrive" size="1.2em" color="--background-color" />`
 */
@Component({
    selector: 'iconify-icon',
    template: `<div [innerHTML]="svgIcon | safeHtml"></div>`,
    styles: [],
    providers: [IconService]
})
export class IconifyIconComponent implements OnInit, OnChanges {
    @Input() icon: string = 'default';
    @Input() size: string | undefined;
    @Input() color: string | undefined;
    svgIcon: string = "";

    constructor(private iconService: IconService) { }

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
