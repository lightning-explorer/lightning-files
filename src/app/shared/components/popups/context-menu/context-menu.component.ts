import { CommonModule } from "@angular/common";
import {
  Component,
  EventEmitter,
  HostListener,
  Input,
  OnDestroy,
  OnInit,
  Output,
} from "@angular/core";
import { ContextMenuButton } from "./models/ContextMenuButton";
import { WindowEventsService } from "@core/services/window/window-events.service";
import { Subscription } from "rxjs";
//
export type ContextMenuItem = ContextMenuButton | {};
@Component({
  selector: "app-context-menu",
  standalone: true,
  imports: [CommonModule],
  templateUrl: "./context-menu.component.html",
  styleUrl: "./context-menu.component.css",
})
export class ContextMenuComponent {
  private subscription = new Subscription();

  @Input() content: ContextMenuItem[] = [];
  /** Emits whenever the context menu gets closed */
  @Output() close = new EventEmitter<void>();
  isVisible = false;
  xPos = 0;
  yPos = 0;

  constructor() {}

  isItemAButton(item: ContextMenuItem): item is ContextMenuButton {
    return (item as ContextMenuButton).action !== undefined;
  }

  toggleOpen(event:MouseEvent) {
    this.xPos = event.clientX;
    this.yPos = event.clientY;
    this.isVisible = !this.isVisible;
  }

  onClick(event: MouseEvent, action: () => void) {
    event.stopPropagation();
    action();
    this.closeMenu();
  }

  @HostListener("document:mousedown", ["$event.target"])
  onClickOutside(targetElement: HTMLElement) {
    const clickedInside = targetElement.closest(".custom-context-menu");
    if (!clickedInside) {
      this.closeMenu();
    }
  }

  @HostListener("document:wheel")
  onWindowScroll() {
    this.closeMenu();
  }

  private closeMenu(){
    this.isVisible = false;
    this.close.emit();
  }
}
