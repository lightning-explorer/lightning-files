import { Component, Input, OnInit } from '@angular/core';
import { FileDTOReceived } from '../../../core/dtos/file-dto-received';
import { CommonModule } from '@angular/common';
import { FileResultComponent } from "../file-result/file-result.component";
import { ScrollingModule } from '@angular/cdk/scrolling';

@Component({
  selector: 'app-files-display',
  standalone: true,
  imports: [CommonModule, FileResultComponent,ScrollingModule],
  templateUrl: './files-display.component.html',
  styleUrl: './files-display.component.scss'
})
export class FilesDisplayComponent {
  @Input() files: FileDTOReceived[] = [];
}
