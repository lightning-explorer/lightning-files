import { ComponentFixture, TestBed } from '@angular/core/testing';

import { CurrentDirectoryBarComponent } from './current-directory-bar.component';

describe('CurrentDirectoryBarComponent', () => {
  let component: CurrentDirectoryBarComponent;
  let fixture: ComponentFixture<CurrentDirectoryBarComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [CurrentDirectoryBarComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(CurrentDirectoryBarComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
