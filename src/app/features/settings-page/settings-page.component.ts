import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { FileCrawlerService } from '../../core/services/files/file_crawler.service';
import { IndexedDirModel } from '../../core/models/indexed-dir-model';
import { CommonModule } from '@angular/common';
import { interval, Subject, switchMap, takeUntil } from 'rxjs';

@Component({
  selector: 'app-settings-page',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './settings-page.component.html',
  styleUrl: './settings-page.component.css'
})
export class SettingsPageComponent implements OnInit {
  crawlerQueue: IndexedDirModel[] = [];
  crawlerPriorityCounts: Array<{ priority: number; count: number }> = [];

  private readonly refreshIntervalMs = 2000; // 2 seconds
  private destroy$ = new Subject<void>();

  constructor(private router: Router, private fileCrawlerService: FileCrawlerService) { }

  async ngOnInit(): Promise<void> {
    interval(this.refreshIntervalMs)
      .pipe(
        takeUntil(this.destroy$),
        switchMap(() => this.refreshCrawlerData())
      )
      .subscribe()
  }

  private async refreshCrawlerData() {
    const priorityCounts = await this.fileCrawlerService.viewCrawlerPriorityCounts();
    this.crawlerPriorityCounts = Object.entries(priorityCounts).map(([priority, count]) => (
      { priority: Number(priority), count }
    ));
  }

  leaveSettingsButtonClick() {
    this.router.navigate(["./home"]);
  }

  ngOnDestroy(): void {
    // Clean up the interval when the component is destroyed
    this.destroy$.next();
    this.destroy$.complete();
  }
}
