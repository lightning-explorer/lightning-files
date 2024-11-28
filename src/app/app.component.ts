import { Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { WindowChromeComponent } from "./layout/window-chrome/window-chrome.component";
import { IconifyIconModule } from './shared/components/IconifyIcons/icon.module';
import { TauriLifecycleService } from './core/services/tauri/lifecycle.service';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, RouterOutlet, WindowChromeComponent, IconifyIconModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss'
})
export class AppComponent implements OnInit {

  constructor(private lifecycleService: TauriLifecycleService) { }

  async ngOnInit() {
    //await this.lifecycleService.onStartup();
  }

}
