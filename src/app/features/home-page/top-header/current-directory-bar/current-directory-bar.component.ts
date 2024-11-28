import { CommonModule } from '@angular/common';
import { AfterViewInit, Component, ElementRef, Input, OnInit, ViewChild } from '@angular/core';
import { FormControl, FormsModule, ReactiveFormsModule } from '@angular/forms';
import { DirectoryNavigatorService } from '../../../../core/services/files/directory-navigator/directory-navigator.service';
import { debounceTime } from 'rxjs';
import { truncateText } from '../../../../core/other/util/text-truncator';
import { simplifyPath } from './util/overflow-checker';

@Component({
  selector: 'app-current-directory-bar',
  standalone: true,
  imports: [CommonModule, FormsModule, ReactiveFormsModule],
  templateUrl: './current-directory-bar.component.html',
  styleUrl: './current-directory-bar.component.scss'
})
export class CurrentDirectoryBarComponent implements AfterViewInit {

  @ViewChild('textInput') textInput!: ElementRef;
  directory = "";
  truncatedDirectory = "";
  hasChanged = false;
  inputControl = new FormControl();

  constructor(private directoryService: DirectoryNavigatorService) { }

  ngAfterViewInit(): void {
    this.directoryService.currentDir$.subscribe(x => {
      this.directory = x;
      this.truncatedDirectory = simplifyPath(x);
    }
    )
  }

  onInputChange() {
    this.hasChanged = true;
  }

  onEnterPressed() {
    this.textInput.nativeElement.blur();
  }

  async onBlur() {
    if (this.hasChanged)
      await this.directoryService.setCurrentDir(this.directory)
    this.hasChanged = false;
  }

  selectText() {
    if (this.textInput && this.textInput.nativeElement) {
      this.textInput.nativeElement.select();
    }
  }
}
