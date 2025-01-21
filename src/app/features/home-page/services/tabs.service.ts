import { Injectable, OnDestroy } from "@angular/core";
import { DirectoryNavigatorService } from "./directory-navigator.service";
import { BehaviorSubject, Subscription } from "rxjs";

@Injectable()
export class TabsService implements OnDestroy {
  private subscription = new Subscription();
  private currentDir = "";

  private openPathsSubject = new BehaviorSubject<string[]>([]);
  openPaths$ = this.openPathsSubject.asObservable();

  constructor(private directoryNavService: DirectoryNavigatorService) {
    this.subscription.add(
      this.directoryNavService.currentDir$.subscribe((x) => {
        // TODO: this could possibly be more robust
        if(x.length!=0){
          this.currentDir = x;
          if (this.openPathsSubject.getValue().length == 0) {
            this.openPathsSubject.next([x]);
          }
        }
      })
    );
  }

  ngOnDestroy(): void {
    this.subscription.unsubscribe;
  }
}
