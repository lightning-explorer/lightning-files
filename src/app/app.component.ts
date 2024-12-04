import { Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { WindowChromeComponent } from "./layout/window-chrome/window-chrome.component";
import { IconifyIconModule } from './shared/components/IconifyIcons/icon.module';
import { TauriLifecycleService } from './core/services/tauri/lifecycle.service';
import { ColorThemeService } from './core/services/customization/color-theme.service';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, RouterOutlet, WindowChromeComponent, IconifyIconModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss'
})
export class AppComponent implements OnInit {

  constructor(private lifecycleService: TauriLifecycleService,
    private themeService: ColorThemeService
  ) { }

  async ngOnInit() {
    this.themeService.setTheme('light-theme');
    //await this.lifecycleService.onStartup();
  }

}
