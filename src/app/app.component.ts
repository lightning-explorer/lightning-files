import { Component, OnInit, ViewEncapsulation } from "@angular/core";
import { CommonModule } from "@angular/common";
import { RouterOutlet } from "@angular/router";
import { IconifyIconModule } from "@shared/components/icons/IconifyIcons/icon.module";
import { ColorThemeService } from "@core/services/customization/color-theme.service";
import { WindowsWindowChromeComponent } from "./layout/windows-window-chrome/windows-window-chrome.component";

@Component({
  selector: "app-root",
  standalone: true,
  imports: [
    CommonModule,
    RouterOutlet,
    IconifyIconModule,
    WindowsWindowChromeComponent,
  ],
  templateUrl: "./app.component.html",
  styleUrl: "./app.component.scss",
})
export class AppComponent implements OnInit {
  constructor(
    private themeService: ColorThemeService
  ) {}

  async ngOnInit() {
    //this.themeService.setTheme("light-theme");
  }
}
