import { ComponentFixture, TestBed } from '@angular/core/testing';

import { HighlightableLabelComponent } from './highlightable-label.component';

describe('HighlightableLabelComponent', () => {
  let component: HighlightableLabelComponent;
  let fixture: ComponentFixture<HighlightableLabelComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [HighlightableLabelComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(HighlightableLabelComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
