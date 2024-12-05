import { CommonModule } from '@angular/common';
import { Component, Input, OnDestroy, OnInit } from '@angular/core';
import { InlineSearchService } from '../../../../../core/services/search/text/inline-search.service';
import { Subscription } from 'rxjs';

@Component({
  selector: 'app-inline-search-bar',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './inline-search-bar.component.html',
  styleUrl: './inline-search-bar.component.css'
})
export class InlineSearchBarComponent implements OnInit, OnDestroy {
  subscription = new Subscription();
  text = "";
  occurences = 0;

  constructor(private inlineSearchService: InlineSearchService) { }

  ngOnInit(): void {
    this.subscription.add(this.inlineSearchService.searchQuery$.subscribe(x =>
      this.text = x
    ));
    this.subscription.add(this.inlineSearchService.numberOfFoundItems$.subscribe(x =>
      this.occurences = x
    ));
  }

  ngOnDestroy(): void {
    this.subscription.unsubscribe();
  }

}
