import { Component } from '@angular/core';
import { TauriLifecycleService } from '../../core/services/tauri/lifecycle.service';
import { getCurrentWindow } from '@tauri-apps/api/window';

/** chrome with square buttons like windows */
@Component({
  selector: 'app-windows-window-chrome',
  standalone: true,
  imports: [],
  templateUrl: './windows-window-chrome.component.html',
  styleUrl: './windows-window-chrome.component.css'
})
export class WindowsWindowChromeComponent {

  constructor(private lifecycleService: TauriLifecycleService) { }

  async closeApp() {
    await this.lifecycleService.onShutdown();
    getCurrentWindow().close();
  }

  minimizeApp() {
    getCurrentWindow().minimize();
  }

  async toggleMaximize() {
    const appWindow = getCurrentWindow();
    const isMaximized = await appWindow.isMaximized();
    if (isMaximized) {
      appWindow.unmaximize();
    } else {
      appWindow.maximize();
    }
  }
}
