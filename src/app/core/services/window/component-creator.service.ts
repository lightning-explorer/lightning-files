import {
  ApplicationRef,
  ComponentFactoryResolver,
  ComponentRef,
  createComponent,
  EnvironmentInjector,
  Injectable,
  Injector,
  Type,
  ViewContainerRef,
} from "@angular/core";

/** Can add components directly to the DOM */
@Injectable({ providedIn: "root" })
export class ComponentCreatorService {
  constructor(
    private injector: EnvironmentInjector,
    private appRef: ApplicationRef
  ) {}

  /** The caller is responsible for destroying the component */
  addToDocumentBody<T>(component: Type<T>): ComponentRef<T> {
    const componentRef = createComponent(component, {
      environmentInjector: this.injector,
    });
    this.appRef.attachView(componentRef.hostView);

    // Get the DOM element of the component and append it to the body
    const domElem = (componentRef.hostView as any).rootNodes[0] as HTMLElement;
    document.body.appendChild(domElem);
    return componentRef;
  }

  removeFromDocumentBody<T>(componentRef: ComponentRef<T> | null) {
    if (componentRef) {
      this.appRef.detachView(componentRef.hostView);

      // Destroy the component instance
      componentRef.destroy();
      componentRef = null;
    }
  }
}
