import { Observable, Subscription } from "rxjs";

/** Reactive subscription manager */
export class RxS {
  private sub = new Subscription();
  constructor() {}

  /** Watch for changes and propagate them to the function */
  watch<T>(fn: Observable<T>, onChanges: (x: T) => void) {
    this.sub.add(
      fn.subscribe((x) => {
        onChanges(x);
      })
    );
  }

  unsub() {
    this.sub.unsubscribe();
  }
}
