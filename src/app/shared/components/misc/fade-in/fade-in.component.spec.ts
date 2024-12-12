import { ComponentFixture, TestBed } from '@angular/core/testing';

import { FadeInComponent } from './fade-in.component';

describe('FadeInComponent', () => {
  let component: FadeInComponent;
  let fixture: ComponentFixture<FadeInComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [FadeInComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(FadeInComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
