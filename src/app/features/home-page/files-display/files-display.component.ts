import { Component, Input, OnChanges, OnInit, SimpleChanges, ViewChild } from '@angular/core';
import { FileDTOReceived } from '../../../core/dtos/file-dto-received';
import { CommonModule } from '@angular/common';
import { FileResultComponent } from "../file-result/file-result.component";
import { CdkVirtualScrollViewport, ScrollingModule } from '@angular/cdk/scrolling';
import { trigger, state, style, animate, transition } from '@angular/animations';
import { FileModel } from '../models/FileModel';
import { InlineSearchService } from '../../../core/services/search/inline-search.service';

@Component({
  selector: 'app-files-display',
  standalone: true,
  imports: [CommonModule, FileResultComponent, ScrollingModule],
  templateUrl: './files-display.component.html',
  styleUrl: './files-display.component.scss',
  animations: [
    trigger('fadeAnimation', [
      state('hidden', style({ opacity: 0, display: 'none' })),
      state('visible', style({ opacity: 1, display: 'block' })),
      transition('hidden => visible', [
        style({ display: 'block' }),
        animate('100ms ease-in')
      ]),
    ])
  ]
})
export class FilesDisplayComponent implements OnInit, OnChanges {
  @ViewChild(CdkVirtualScrollViewport) viewport!: CdkVirtualScrollViewport;
  @Input() files: FileModel[] = [];

  animationState = 'visible';

  constructor(private inlineSearchService: InlineSearchService) { }

  ngOnInit(): void {
    this.inlineSearchService.firstOccurenceOfQueryIndex$.subscribe(x =>
      this.inlineSearchToFirstOccurence(x)
    )
  }

  ngOnChanges(changes: SimpleChanges) {
    if (changes['files']) {
      this.hideAndFadeIn();
    }
  }

  hideAndFadeIn() {
    this.animationState = 'hidden';

    setTimeout(() => {
      this.animationState = 'visible';
      this.viewport.checkViewportSize();
    }, 100); // Match this to the duration of the hide animation
  }

  // scroll to the first occurence of a file/directory with offset
  inlineSearchToFirstOccurence(index: number) {
    if (this.viewport) {
      let indexOffset = index - 6;
      if (indexOffset < 1)
        indexOffset = 0;
      this.viewport.scrollToIndex(indexOffset, 'smooth');
    }
  }
}
