import {
  ComponentRef,
  Directive,
  HostListener,
  Input,
  OnDestroy,
  ViewContainerRef,
} from "@angular/core";
import { TooltipComponent } from "./tooltip.component";

@Directive({ selector: "[appTooltip]", standalone: true })
export class TooltipDirective implements OnDestroy {
  @Input("appTooltip") text: string = "";
  private componentRef: ComponentRef<TooltipComponent> | null = null;

  constructor(private viewContainer: ViewContainerRef) {}

  @HostListener("mouseenter") onMouseEnter() {
    if (!this.componentRef) {
      this.componentRef = this.viewContainer.createComponent(TooltipComponent);
      this.componentRef.instance.text = this.text;
    }
  }

  @HostListener("mouseleave") onMouseLeave() {
    if (this.componentRef) {
      this.componentRef.destroy();
      this.componentRef = null;
    }
  }

  ngOnDestroy() {
    if (this.componentRef) {
      this.componentRef.destroy();
    }
  }
}
