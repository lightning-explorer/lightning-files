import { ComponentFixture, TestBed } from '@angular/core/testing';

import { QuickAccessShortcutComponent } from './quick-access-shortcut.component';

describe('QuickAccessShortcutComponent', () => {
  let component: QuickAccessShortcutComponent;
  let fixture: ComponentFixture<QuickAccessShortcutComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [QuickAccessShortcutComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(QuickAccessShortcutComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
