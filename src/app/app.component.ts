import { Component, OnInit, ViewEncapsulation } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { WindowChromeComponent } from "./layout/window-chrome/window-chrome.component";
import { IconifyIconModule } from '@shared/components/IconifyIcons/icon.module';
import { TauriLifecycleService } from '@core/services/tauri/lifecycle.service';
import { ColorThemeService } from '@core/services/customization/color-theme.service';
import { WindowsWindowChromeComponent } from "./layout/windows-window-chrome/windows-window-chrome.component";

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, RouterOutlet, WindowChromeComponent, IconifyIconModule, WindowsWindowChromeComponent],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss',
})
export class AppComponent implements OnInit {

  constructor(private lifecycleService: TauriLifecycleService,
    private themeService: ColorThemeService
  ) { }

  async ngOnInit() {
    //this.themeService.setTheme('light-theme2');
    await this.lifecycleService.onStartup();
  }

}
