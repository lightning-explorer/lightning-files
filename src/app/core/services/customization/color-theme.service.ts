import { Injectable, Renderer2, RendererFactory2 } from '@angular/core';

@Injectable({
  providedIn: 'root',
})
export class ColorThemeService {
  private renderer: Renderer2;
  private themeClass: string | null = null;

  constructor(private rendererFactory: RendererFactory2) {
    this.renderer = this.rendererFactory.createRenderer(null, null);
  }

  setTheme(themeName: string): void {
    if (this.themeClass) {
      this.renderer.removeClass(document.body, this.themeClass);
    }
    this.themeClass = themeName;
    this.renderer.addClass(document.body, themeName);
  }

  getCurrentTheme(): string | null {
    return this.themeClass;
  }
}