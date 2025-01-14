import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MutedButtonComponent } from './muted-button.component';

describe('MutedButtonComponent', () => {
  let component: MutedButtonComponent;
  let fixture: ComponentFixture<MutedButtonComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MutedButtonComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(MutedButtonComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
