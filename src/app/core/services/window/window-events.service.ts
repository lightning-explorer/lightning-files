import { Injectable, NgZone } from "@angular/core";
import { Subject } from "rxjs";

@Injectable({ providedIn: "root" })
export class WindowEventsService {
  private mouseEnterSubject = new Subject<void>();
  /** Emits whenever the user's mouse enters the app window */
  mouseEnter$ = this.mouseEnterSubject.asObservable();

  private mouseLeaveSubject = new Subject<void>();
  /** Emits whenever the user's mouse leaves the app window */
  mouseLeave$ = this.mouseLeaveSubject.asObservable();

  private isMouseInside = false;

  constructor(private zone: NgZone) {
    // Use NgZone to avoid Angular's change detection triggering for every mousemove
    this.zone.runOutsideAngular(() => {
      //window.addEventListener("mousemove", this.onMouseMove.bind(this));
      window.addEventListener("dragleave", this.onMouseMove.bind(this));
    });
  }

  private onMouseMove(event: MouseEvent | DragEvent) {
    console.log('dragin');
    const { clientX, clientY } = event;
    const isInside =
      clientX >= 0 &&
      clientY >= 0 &&
      clientX <= window.innerWidth &&
      clientY <= window.innerHeight;


    if (isInside && !this.isMouseInside) {
      this.isMouseInside = true;
      this.zone.run(() => this.mouseEnterSubject.next());
      console.log('m enter');
    } else if (!isInside && this.isMouseInside) {
      this.isMouseInside = false;
      this.zone.run(() => this.mouseLeaveSubject.next());
      console.log('m leave');
    }
  }
}