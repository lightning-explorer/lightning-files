import { Component, OnDestroy, OnInit } from "@angular/core";
import { Subscription } from "rxjs";
import { CommonModule } from "@angular/common";
import { DirectoryTabComponent } from "./directory-tab/directory-tab.component";
import { TabsService } from "src/app/features/home-page/services/tabs.service";
import { FileNameResolverService } from "@core/services/files/name-resolver.service";
import { PrettyButtonComponent } from "../../../../../../../shared/components/buttons/pretty-button/pretty-button.component";

export interface DirectoryTabModel {
  fullPath: string;
  label: string;
}

@Component({
  selector: "app-tabs-holder",
  standalone: true,
  imports: [CommonModule, DirectoryTabComponent, PrettyButtonComponent],
  templateUrl: "./tabs-holder.component.html",
  styleUrl: "./tabs-holder.component.css",
})
export class TabsHolderComponent implements OnInit, OnDestroy {
  private subscription = new Subscription();

  tabs: DirectoryTabModel[] = [];

  constructor(
    private tabsService: TabsService,
    private nameResolverService: FileNameResolverService
  ) {}

  ngOnInit(): void {
    this.subscription.add(
      this.tabsService.openPaths$.subscribe((paths) => {
        this.tabs = paths.map((x) => this.filePathToTabModel(x));
      })
    );
  }

  ngOnDestroy(): void {
    this.subscription.unsubscribe();
  }

  test() {
    console.log("et");
  }

  filePathToTabModel(path: string): DirectoryTabModel {
    const label = this.nameResolverService.getFileNameFromFullPath(path);
    return {
      fullPath: path,
      label,
    };
  }
}
