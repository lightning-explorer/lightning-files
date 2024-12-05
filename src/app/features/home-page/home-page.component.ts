import { Component, HostListener, OnInit } from '@angular/core';

import { CommonModule } from '@angular/common';

import { SidebarComponent } from "./sidebar/sidebar.component";
import { FilesDisplayComponent } from "./files-display/files-display.component";
;
import { MatIconModule } from '@angular/material/icon';

import { TopHeaderComponent } from "./top-header/top-header.component";
import { PinnedFilesHeaderComponent } from "./pinned-files-header/pinned-files-header.component";

// TODO:
// put search bar in Shared and then make a simpler one in features to manage its own state

@Component({
  selector: 'app-home-page',
  standalone: true,
  imports: [CommonModule, SidebarComponent, FilesDisplayComponent, MatIconModule, TopHeaderComponent, PinnedFilesHeaderComponent],
  templateUrl: './home-page.component.html',
  styleUrl: './home-page.component.scss',
  providers: []
})
export class HomePageComponent implements OnInit {


  constructor(

  ) { }

  ngOnInit(): void {

  }



}
