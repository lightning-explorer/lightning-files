import { Component, Input, OnInit } from '@angular/core';
import { FileDTOReceived } from '../../../core/dtos/file-dto-received';
import { CommonModule } from '@angular/common';
import { FileResultComponent } from "../file-result/file-result.component";
import { Observable } from 'rxjs';

@Component({
  selector: 'app-files-display',
  standalone: true,
  imports: [CommonModule, FileResultComponent,],
  templateUrl: './files-display.component.html',
  styleUrl: './files-display.component.scss'
})
export class FilesDisplayComponent {
  @Input() files: FileDTOReceived[] = [];

  trackByFile(index: number, file: FileDTOReceived): string {
    return file.Name; // or a unique property in your FileDTO that can uniquely identify each file
  }
}
