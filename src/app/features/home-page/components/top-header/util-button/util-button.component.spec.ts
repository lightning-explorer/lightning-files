import { ComponentFixture, TestBed } from '@angular/core/testing';

import { UtilButtonComponent } from './util-button.component';

describe('UtilButtonComponent', () => {
  let component: UtilButtonComponent;
  let fixture: ComponentFixture<UtilButtonComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [UtilButtonComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(UtilButtonComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
