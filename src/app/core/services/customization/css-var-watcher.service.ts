import { Injectable } from '@angular/core';
import { BehaviorSubject, Observable } from 'rxjs';

@Injectable({
    providedIn: 'root',
})
export class CssVarWatcherService {
    private cssVariableSubject: BehaviorSubject<Record<string, string>> = new BehaviorSubject({});
    /**
     * Subscribing to this allows the component to be notified if any of the global CSS variables change 
      */
    cssVariables$ = this.cssVariableSubject.asObservable();
    private observer: MutationObserver | undefined;

    constructor() {
        this.startObserving();
    }

    private startObserving() {
        const root = document.documentElement;

        // Initialize with current CSS variables
        this.updateCssVariables();

        this.observer = new MutationObserver(() => {
            this.updateCssVariables();
        });

        // Observe changes to inline styles on the root element
        this.observer.observe(root, {
            attributes: true,
            attributeFilter: ['style']
        });
    }

    private updateCssVariables() {
        const root = getComputedStyle(document.documentElement);
        const cssVariables: Record<string, string> = {};

        for (let i = 0; i < root.length; i++) {
            const name = root[i];
            if (name.startsWith('--')) {
                cssVariables[name] = root.getPropertyValue(name).trim();
            }
        }
        console.log("cjange");
        this.cssVariableSubject.next(cssVariables);
    }

    ngOnDestroy(): void {
        if (this.observer) {
            this.observer.disconnect();
        }
    }
}