import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ButtonWSvgComponent } from './button-w-svg.component';

describe('ButtonWSvgComponent', () => {
  let component: ButtonWSvgComponent;
  let fixture: ComponentFixture<ButtonWSvgComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ButtonWSvgComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(ButtonWSvgComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
