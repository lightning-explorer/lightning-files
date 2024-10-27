import { Component, Input } from '@angular/core';
import { FileDTOReceived } from '../../../core/services/dtos/file-dto-received';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-file-result',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './file-result.component.html',
  styleUrl: './file-result.component.scss'
})
export class FileResultComponent {
  @Input() file: FileDTOReceived | undefined;
}
