import { Component, Input } from '@angular/core';
import { FileIconComponent } from "../../../../../file-icon/file-icon.component";
import { DirectoryTabModel } from '../tabs-holder.component';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-directory-tab',
  standalone: true,
  imports: [FileIconComponent, CommonModule],
  templateUrl: './directory-tab.component.html',
  styleUrl: './directory-tab.component.css'
})
export class DirectoryTabComponent {
  @Input() tab?:DirectoryTabModel;

  ngOnInit(){
    console.log(this.tab?.fullPath);
  }

  onClick(){
    console.log('tab clicked');
  }
}
