import { Component, ElementRef, HostBinding, OnInit } from '@angular/core';
import { trigger, state, style, transition, animate } from '@angular/animations';
import { CommonModule } from '@angular/common';

/** 
 *  A component that fades in when it becomes visible to the user
 * 
 *  ### Usage example:
 * 
 * `<app-fade-in>
 *     <div/>
 * </app-fade-in>`
*/
@Component({
  selector: 'app-fade-in',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './fade-in.component.html',
  styleUrl: './fade-in.component.css',
  animations: [
    trigger('fadeIn', [
      state('hidden', style({ opacity: 0 })),
      state('visible', style({ opacity: 1 })),
      transition('hidden => visible', [animate('0.2s ease-in')]),
    ])
  ]
})
export class FadeInComponent implements OnInit {
  @HostBinding('@fadeIn') animationState = 'hidden'; // Initial animation state
  isVisible = false;

  constructor(private el: ElementRef) { }

  ngOnInit() {
    const observer = new IntersectionObserver(
      (entries) => {
        const entry = entries[0];
        if (entry.isIntersecting) {
          this.isVisible = true;
          this.animationState = 'visible';
          observer.disconnect(); // Stop observing after animation starts
        }
      },
      { threshold: 0.1 } // Adjust threshold as needed
    );

    observer.observe(this.el.nativeElement);
  }
}
