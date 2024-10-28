import { CommonModule } from '@angular/common';
import { Component, ElementRef, Input, OnInit, ViewChild } from '@angular/core';
import { FormControl, FormsModule, ReactiveFormsModule } from '@angular/forms';
import { DirectoryNavigatorService } from '../../../core/services/directory-navigator.service';
import { debounceTime } from 'rxjs';

@Component({
  selector: 'app-current-directory-bar',
  standalone: true,
  imports: [CommonModule, FormsModule, ReactiveFormsModule],
  templateUrl: './current-directory-bar.component.html',
  styleUrl: './current-directory-bar.component.scss'
})
export class CurrentDirectoryBarComponent implements OnInit {

  @ViewChild('textInput') textInput!: ElementRef;
  directory = "";
  hasChanged = false;
  inputControl = new FormControl();


  constructor(private directoryService: DirectoryNavigatorService) { }

  ngOnInit(): void {
    this.directoryService.currentDir$.subscribe(x =>
      this.directory = x
    )
  }

  onInputChange() {
    this.hasChanged = true;
  }

  onEnterPressed(){
    this.textInput.nativeElement.blur();
  }

  onBlur(){
    if (this.hasChanged)
      this.directoryService.setCurrentDir(this.directory)
    this.hasChanged = false;
  }

  selectText() {
    if (this.textInput && this.textInput.nativeElement) {
      this.textInput.nativeElement.select();
    }
  }
}
