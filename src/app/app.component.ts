import { Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { getCurrentWindow } from "@tauri-apps/api/window";
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
    const window = getCurrentWindow();

    // TODO: Revisit this code as it causes the backend to panic if the frontend initializes before it does
    // await this.lifecycleService.onStartup();
  }

}
