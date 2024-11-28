import { ComponentFixture, TestBed } from '@angular/core/testing';

import { InlineSearchBarComponent } from './inline-search-bar.component';

describe('InlineSearchBarComponent', () => {
  let component: InlineSearchBarComponent;
  let fixture: ComponentFixture<InlineSearchBarComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [InlineSearchBarComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(InlineSearchBarComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
