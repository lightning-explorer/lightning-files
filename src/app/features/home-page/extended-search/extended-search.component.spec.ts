import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ExtendedSearchComponent } from './extended-search.component';

describe('ExtendedSearchComponent', () => {
  let component: ExtendedSearchComponent;
  let fixture: ComponentFixture<ExtendedSearchComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ExtendedSearchComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(ExtendedSearchComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
