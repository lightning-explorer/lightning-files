import { CommonModule } from '@angular/common';
import { Component, HostListener, Input } from '@angular/core';
import { ContextMenuButton } from './models/ContextMenuButton';

@Component({
  selector: 'app-context-menu',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './context-menu.component.html',
  styleUrl: './context-menu.component.css'
})
export class ContextMenuComponent {
  @Input() buttons: ContextMenuButton[] = [];

  isVisible = false;
  xPos = 0;
  yPos = 0;

  toggleOpen(event: MouseEvent) {
    this.isVisible = !this.isVisible;
    this.xPos = event.clientX;
    this.yPos = event.clientY;
  }

  onClick(event: MouseEvent, action: () => void) {
    event.stopPropagation();
    action();
    this.isVisible = false;
  }

  @HostListener('document:mousedown', ['$event.target'])
  onClickOutside(targetElement: HTMLElement) {
    const clickedInside = targetElement.closest('.custom-context-menu');
    if (!clickedInside) {
      this.isVisible = false;
    }
  }

  @HostListener('document:wheel')
  onWindowScroll() {
    this.isVisible = false;
  }
}