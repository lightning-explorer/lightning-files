import { Component, Input, OnInit } from '@angular/core';
import { FormControl, ReactiveFormsModule } from '@angular/forms';
import { debounceTime } from 'rxjs';

@Component({
  selector: 'app-searchbar',
  standalone: true,
  imports: [ReactiveFormsModule],
  templateUrl: './searchbar.component.html',
  styleUrl: './searchbar.component.scss'
})
export class SearchbarComponent implements OnInit {

  inputControl = new FormControl();
  @Input() searchFunction?: (input: string)=>void;
  @Input() blurFunction?: ()=>void;

  ngOnInit(): void {

    if (this.searchFunction) {
      this.inputControl.valueChanges.pipe(
        debounceTime(100)
      ).subscribe(value =>
        this.searchFunction!(value)
      )
    }
  }

  onBlur(){
    if(this.blurFunction)
      this.blurFunction()
  }

}
