import { Component } from '@angular/core';
import { getCurrentWindow } from "@tauri-apps/api/window";

@Component({
  selector: 'app-window-chrome',
  standalone: true,
  imports: [],
  templateUrl: './window-chrome.component.html',
  styleUrl: './window-chrome.component.scss'
})
export class WindowChromeComponent {
  closeApp() {
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
