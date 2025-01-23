import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DragCollectButtonComponent } from './drag-collect-button.component';

describe('DragCollectButtonComponent', () => {
  let component: DragCollectButtonComponent;
  let fixture: ComponentFixture<DragCollectButtonComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [DragCollectButtonComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(DragCollectButtonComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
