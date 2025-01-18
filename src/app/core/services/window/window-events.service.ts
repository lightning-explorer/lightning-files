import { HostListener, Injectable, NgZone } from "@angular/core";
import { BehaviorSubject } from "rxjs";

interface Dimensions{
  width:number,
  height:number
}
interface Point{
  x:number,
  y:number
}

@Injectable({ providedIn: "root" })
export class WindowEventsService {
  private windowDimensionsSubject = new BehaviorSubject<Dimensions>({width:window.innerWidth,height:window.innerHeight});
  windowDimensions$ = this.windowDimensionsSubject.asObservable();

  private cursorPositionSubject = new BehaviorSubject<Point>({x:0,y:0});
  cursorPosition$ = this.cursorPositionSubject.asObservable();

  constructor(private zone:NgZone){
    this.zone.runOutsideAngular(() => {
      window.addEventListener("resize", this.onResize.bind(this));
      window.addEventListener("mousemove",this.onMouseMove.bind(this));
    });
  }

  private onResize(event: Event): void {
    this.updateWindowSize();
  }

  private onMouseMove(event:MouseEvent){
    this.cursorPositionSubject.next({x:event.clientX,y:event.clientY});
  }

  private updateWindowSize(): void {
    this.windowDimensionsSubject.next({width: window.innerWidth, height: window.innerHeight});
  }
}
