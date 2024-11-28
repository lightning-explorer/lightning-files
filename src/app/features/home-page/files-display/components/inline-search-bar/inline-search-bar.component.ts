import { CommonModule } from '@angular/common';
import { Component, Input, OnInit } from '@angular/core';
import { InlineSearchService } from '../../../../../core/services/search/text/inline-search.service';

@Component({
  selector: 'app-inline-search-bar',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './inline-search-bar.component.html',
  styleUrl: './inline-search-bar.component.css'
})
export class InlineSearchBarComponent implements OnInit {
  text = "";
  occurences = 0;

  constructor(private inlineSearchService: InlineSearchService) { }

  ngOnInit(): void {
    this.inlineSearchService.searchQuery$.subscribe(x =>
      this.text = x
    );
    this.inlineSearchService.numberOfFoundItems$.subscribe(x =>
      this.occurences = x
    );
  }

}
