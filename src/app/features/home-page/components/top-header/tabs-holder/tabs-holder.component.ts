import { Component, OnDestroy, OnInit } from '@angular/core';
import { TabsService } from '../../../services/tabs.service';
import { Subscription } from 'rxjs';
import { CommonModule } from '@angular/common';
import { DirectoryTabComponent } from "./directory-tab/directory-tab.component";

interface TabModel{
  fullPath:string,
  label:string,
}

@Component({
  selector: 'app-tabs-holder',
  standalone: true,
  imports: [CommonModule, DirectoryTabComponent],
  templateUrl: './tabs-holder.component.html',
  styleUrl: './tabs-holder.component.css'
})
export class TabsHolderComponent implements OnInit, OnDestroy{
  private subscription = new Subscription();

  tabs:TabModel[] = [];

  constructor(private tabsService:TabsService){}

  ngOnInit(): void {
      this.subscription.add(this.tabsService.openPaths$.subscribe(paths=>{
        this.tabs = paths.map(x=>this.filePathToTabModel(x));
      }));
  }

  ngOnDestroy(): void {
    this.subscription.unsubscribe();
  }

  test(){
    console.log('et');
  }

  filePathToTabModel(path:string):TabModel{
    return {
      fullPath: path,
      label:'label'
    }
  }
}
