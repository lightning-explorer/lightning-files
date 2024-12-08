import { CommonModule } from '@angular/common';
import { AfterViewInit, ChangeDetectorRef, Component, ElementRef, HostListener, Input, OnDestroy, OnInit, ViewChild } from '@angular/core';
import { FormControl, FormsModule, ReactiveFormsModule } from '@angular/forms';
import { DirectoryNavigatorService } from '../../../../core/services/files/directory-navigator/directory-navigator.service';
import { Subscription } from 'rxjs';
import { BreadcrumbModel } from './models/breadcrumb-model';

@Component({
  selector: 'app-current-directory-bar',
  standalone: true,
  imports: [CommonModule, FormsModule, ReactiveFormsModule],
  templateUrl: './current-directory-bar.component.html',
  styleUrl: './current-directory-bar.component.scss'
})
export class CurrentDirectoryBarComponent implements AfterViewInit, OnInit, OnDestroy {
  subscription = new Subscription();

  @ViewChild('textInput') textInput!: ElementRef;

  directory = "";
  visibleDirectories: BreadcrumbModel[] = [];
  showEllipsis: boolean = false;

  hasChanged = false;
  inputControl = new FormControl();

  constructor(private directoryService: DirectoryNavigatorService, private cdr: ChangeDetectorRef) { }

  ngOnInit(): void {
    this.updateVisibleDirectories();
  }

  ngAfterViewInit(): void {
    this.subscription.add(this.directoryService.currentDir$.subscribe(x => {
      this.directory = x;
      this.updateVisibleDirectories();
    }));
  }

  @HostListener('window:resize')
  onResize() {
    this.updateVisibleDirectories();
  }

  ngOnDestroy(): void {
    this.subscription.unsubscribe();
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

  async onBreadcrumbClicked(model: BreadcrumbModel) {
    await this.directoryService.setCurrentDir(model.fullPath);
  }

  selectText() {
    if (this.textInput && this.textInput.nativeElement) {
      this.textInput.nativeElement.select();
    }
  }

  private updateVisibleDirectories() {
    const parts = this.directory.split('\\');
    let dirBuilder: string = "";
    this.visibleDirectories.length = 0;
    parts.forEach(x => {
      dirBuilder += `${x}\\`;
      this.visibleDirectories.push({ fullPath: dirBuilder, section: x });
    });

    this.cdr.detectChanges();

    const containerWidth = this.textInput.nativeElement.offsetWidth;
    const breadcrumbElements = Array.from(this.textInput.nativeElement.querySelectorAll('.breadcrumb')) as HTMLElement[];
    const padding = 1.2; // Arbitrary padding
    const elementWidths = breadcrumbElements.map(element => element.offsetWidth * padding);
    const totalWidth = elementWidths.reduce((sum, width) => sum + width, 0);

    let currentWidth = totalWidth;

    while (currentWidth > containerWidth && this.visibleDirectories.length > 1) {
      currentWidth -= elementWidths.shift()!;
      this.visibleDirectories.shift();
    }

    this.showEllipsis = this.visibleDirectories.length > 0;
  }
}
