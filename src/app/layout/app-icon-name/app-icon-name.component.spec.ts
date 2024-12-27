import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AppIconNameComponent } from './app-icon-name.component';

describe('AppIconNameComponent', () => {
  let component: AppIconNameComponent;
  let fixture: ComponentFixture<AppIconNameComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [AppIconNameComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(AppIconNameComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
