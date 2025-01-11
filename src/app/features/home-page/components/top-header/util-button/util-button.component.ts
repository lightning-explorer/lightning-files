import { AfterViewInit, Component, ElementRef, Input, OnInit } from '@angular/core';
import { IconifyIconModule } from "../../../../../shared/components/icons/IconifyIcons/icon.module";
import { CommonModule } from '@angular/common';
import { DropdownButtonModalComponent } from "../../../../../shared/components/buttons/dropdown-button-modal/dropdown-button-modal.component";

export type UtilButtonType = 'copy' | 'paste' | 'new' | 'rename' | 'trash' | 'cut' | 'more';
@Component({
  selector: 'app-util-button',
  standalone: true,
  imports: [IconifyIconModule, CommonModule, DropdownButtonModalComponent],
  templateUrl: './util-button.component.html',
  styleUrl: './util-button.component.css'
})
export class UtilButtonComponent implements OnInit, AfterViewInit {
  _dropdownFeatures = [];
  _isDropdownType = false;
  _icon:string|undefined = undefined;

  constructor(private elementRef:ElementRef){}

  @Input() type:UtilButtonType = 'copy';

  ngOnInit(): void {
    if(this.type=='new')
      this._isDropdownType = true;
    this._icon = this.type;
  }

  ngAfterViewInit(): void {
    //const e = this.elementRef.nativeElement as HTMLElement;
    //console.log(e.offsetWidth)
  }
}
