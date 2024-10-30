import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ButtonWIconComponent } from './button-w-icon.component';

describe('ButtonWIconComponent', () => {
  let component: ButtonWIconComponent;
  let fixture: ComponentFixture<ButtonWIconComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ButtonWIconComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(ButtonWIconComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
