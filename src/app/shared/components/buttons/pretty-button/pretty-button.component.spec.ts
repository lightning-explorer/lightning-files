import { ComponentFixture, TestBed } from '@angular/core/testing';

import { PrettyButtonComponent } from './pretty-button.component';

describe('PrettyButtonComponent', () => {
  let component: PrettyButtonComponent;
  let fixture: ComponentFixture<PrettyButtonComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [PrettyButtonComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(PrettyButtonComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
