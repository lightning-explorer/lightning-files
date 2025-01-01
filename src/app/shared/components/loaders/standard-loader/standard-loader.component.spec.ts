import { ComponentFixture, TestBed } from '@angular/core/testing';

import { StandardLoaderComponent } from './standard-loader.component';

describe('StandardLoaderComponent', () => {
  let component: StandardLoaderComponent;
  let fixture: ComponentFixture<StandardLoaderComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [StandardLoaderComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(StandardLoaderComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
